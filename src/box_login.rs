use std::{
    collections::HashMap,
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Termination,
};

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
};
use r#box::{
    apis::{
        authorization_api::{GetAuthorizeParams, PostOauth2TokenParams},
        configuration::Configuration,
    },
    models::AccessToken,
};
use port_check::free_local_port_in_range;
use rand::{Rng, distr::Alphanumeric};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};

use anyhow::{Ok, Result};
use tracing::{debug, error};

const TOKEN_URL: &str = "https://api.box.com/oauth2/token";

pub async fn get_key(
    id: String,
    secret: String,
    config_path: PathBuf,
    read_cache: bool,
) -> anyhow::Result<AccessToken> {
    let client_id = id;
    let client_secret = secret;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(config_path)
        .await?;

    let mut s = String::new();
    if read_cache {
        file.read_to_string(&mut s).await?;
    }
    let token: AccessToken = match serde_json::from_str::<AccessToken>(&s) {
        std::result::Result::Ok(t) => t,
        Err(_) => {
            let port = free_local_port_in_range(8080..=8090)
                .ok_or(anyhow::anyhow!("No free ports available"))?;

            let redirect_uri = format!("http://localhost:{port}");
            let state = generate_random_state();
            let oauth_url = format!(
                "https://account.box.com/api/oauth2/authorize?client_id={client_id}&redirect_uri={redirect_uri}&state={state}&response_type=code"
            );

            // Create a channel to receive the authorization code
            let (code_sender, mut code_receiver) = mpsc::channel::<Option<(String, String)>>(1);
            let state_clone = state.clone();

            // Build the Axum router
            let app = axum::Router::new()
                .route("/", get(handle_callback))
                .with_state((code_sender, state_clone));

            // Start the local server
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let server = axum::serve(tokio::net::TcpListener::bind(addr).await?, app);

            // Start the server in a separate task
            let server_handle = tokio::spawn(async move { server.await });

            if let Err(e) = webbrowser::open(&oauth_url) {
                error!("Failed to open browser: {}", e);
                error!("Please open this URL manually:\n{}", oauth_url);
            }

            // Wait for the callback
            debug!("Waiting for authentication callback...");
            let k = match match code_receiver.recv().await {
                Some(Some((code, n_state))) => {
                    // Verify state to prevent CSRF

                    if state != n_state {
                        return Err(anyhow::anyhow!("State Mismatch"));
                    }

                    exchange_code_for_token(&code, &client_id, &client_secret, &redirect_uri).await
                }
                Some(None) => {
                    return Err(anyhow::anyhow!("Authentication was cancelled or failed."));
                }
                None => return Err(anyhow::anyhow!("Error receiving authentication code")),
            } {
                std::result::Result::Ok(k) => {
                    file.set_len(0).await?;
                    let token_json = serde_json::to_string_pretty(&k)?;
                    file.write_all(token_json.as_bytes()).await?;
                    k
                }
                Err(e) => {
                    return Err(e);
                }
            };
            k
        }
    };
    Ok(token)
}

async fn handle_callback(
    Query(params): Query<HashMap<String, String>>,
    axum::extract::State((code_sender, expected_state)): axum::extract::State<(
        mpsc::Sender<Option<(String, String)>>,
        String,
    )>,
) -> Result<impl axum::response::IntoResponse, ()> {
    let html_success = r#"
        <html>
            <body>
                <h1>Authentication Successful!</h1>
                <p>You can close this window and return to the application.</p>
            </body>
        </html>
    "#;

    let html_error = r#"
        <html>
            <body>
                <h1>Authentication Failed</h1>
                <p>There was an error during authentication. Please try again.</p>
            </body>
        </html>
    "#;

    // Check for error first
    if let Some(error) = params.get("error") {
        eprintln!(
            "OAuth error: {} - {:?}",
            error,
            params.get("error_description")
        );
        let _ = code_sender.send(None);
        return std::result::Result::Ok(Html(html_error).into_response());
    }

    // Extract authorization code and state
    if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
        // Verify state matches what we expect
        if state != &expected_state {
            eprintln!("State mismatch: expected {}, got {}", expected_state, state);
            let _ = code_sender.send(None);
            return std::result::Result::Ok(Html(html_error).into_response());
        }

        // Send the code back to the main task
        if code_sender
            .send(Some((code.clone(), state.clone())))
            .await
            .is_ok()
        {
            std::result::Result::Ok(Html(html_success).into_response())
        } else {
            eprintln!("Failed to send authorization code");
            std::result::Result::Ok(Html(html_error).into_response())
        }
    } else {
        eprintln!("Missing code or state in callback");
        let _ = code_sender.send(None);
        std::result::Result::Ok(Html(html_error).into_response())
    }
}

fn generate_random_state() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

async fn exchange_code_for_token(
    code: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> anyhow::Result<AccessToken> {
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("redirect_uri", redirect_uri),
    ];

    let response = client.post(TOKEN_URL).form(&params).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Token exchange failed: {}", error_text).into());
    }

    let token_response: AccessToken = response.json().await?;
    Ok(token_response)
}
