use std::path::{Path, PathBuf};
use std::vec;

use anyhow::Ok;
use async_trait::async_trait;
use google_sheets4::api::{
    AppendCellsRequest, BatchUpdateSpreadsheetRequest, CellData, ExtendedValue, Request, RowData,
};
use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;
use google_sheets4::yup_oauth2::ApplicationSecret;
use google_sheets4::yup_oauth2::authenticator_delegate::InstalledFlowDelegate;
use google_sheets4::yup_oauth2::error::TokenStorageError;
use google_sheets4::yup_oauth2::storage::{TokenInfo, TokenStorage};
use google_sheets4::{FieldMask, Sheets, hyper_rustls, hyper_util, yup_oauth2};
use port_check::free_local_port_in_range;
use tracing::info;

use crate::persist::persist;

struct BrowserDelegate;
impl InstalledFlowDelegate for BrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(async move {
            webbrowser::open(url)
                .map(|_| "".to_string())
                .map_err(|e| e.to_string())
        })
    }
}

pub async fn google_login(
    client_id: String,
    client_secret: String,
    persist_file: PathBuf
) -> anyhow::Result<Sheets<HttpsConnector<HttpConnector>>> {
    let port = free_local_port_in_range(8080..=8090).ok_or(anyhow::format_err!("No free port available"))?;
    let secret = ApplicationSecret {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        token_uri: "https://oauth2.googleapis.com/token".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        redirect_uris: vec![format!("http://localhost")],
        project_id: Some("TagMonster".to_string()),
        client_email: None,
        auth_provider_x509_cert_url: None,
        client_x509_cert_url: None,
    };
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_only()
        .enable_http2()
        .build();
    let executor = hyper_util::rt::TokioExecutor::new();
    let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPPortRedirect(port),
        yup_oauth2::client::CustomHyperClientBuilder::from(
            hyper_util::client::legacy::Client::builder(executor).build(connector),
        ),
    )
    .flow_delegate(Box::new(BrowserDelegate))
    .persist_tokens_to_disk(persist_file)
    .build()
    .await?;
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()?
                .https_or_http()
                .enable_http2()
                .build(),
        );

    info!("Setup Google API client");

    Ok(Sheets::new(client, auth))
}
