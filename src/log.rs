use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, registry::Registry};

use crate::CONFIG_DIR;

struct FieldVisitor {
    fields: BTreeMap<String, String>,
}

impl FieldVisitor {
    fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{:?}", value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.fields
            .insert(field.name().to_string(), format!("{:?}", value));
    }

    // Fallback for other numeric types
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }
}

/// Layer that sends formatted log lines to a GUI via a channel.
pub struct GuiLayer {
    sender: Arc<Sender<(String, tracing::Level)>>,
}

impl GuiLayer {
    pub fn new(sender: Sender<(String, tracing::Level)>) -> Self {
        Self {
            sender: Arc::new(sender),
        }
    }
}

impl<S: Subscriber> Layer<S> for GuiLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let meta = event.metadata();
        let line = meta.line();
        let location = meta.file().map(|f| {
            if let Some(line) = line {
                format!("{f}:{line}")
            } else {
                f.to_string()
            }
        });
        let level = meta.level();
        let target = meta.target();

        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let payload = if let Some(msg) = visitor.fields.get("message").cloned() {
            msg
        } else if let Some(msg) = visitor.fields.get("msg").cloned() {
            msg
        } else if visitor.fields.is_empty() {
            meta.name().to_string()
        } else {
            visitor
                .fields
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(" ")
        };

        let level_str = match *level {
            Level::ERROR => "[ERROR]",
            Level::WARN => " [WARN]",
            Level::INFO => " [INFO]",
            Level::DEBUG => "[DEBUG]",
            Level::TRACE => "[TRACE]",
        };

        let line = (
            format!(
                "{} {} - {}",
                level_str,
                if let Some(l) = location {
                    l
                } else {
                    target.to_string()
                },
                payload
            ),
            *level,
        );

        let _ = self.sender.send(line);
    }
}

/// Keeps trace appender worker guard alive so file logging actually flushes.
pub struct LoggingGuard {
    _worker_guard: WorkerGuard,
}

impl LoggingGuard {
    fn new(worker_guard: WorkerGuard) -> Self {
        Self {
            _worker_guard: worker_guard,
        }
    }
}

/// Initialize logging:
/// - writes to a rolling daily file under "logs/tagmonster.log"
/// - writes to stderr
/// - sends formatted log lines to the provided GUI channel
///
/// Returns a LoggingGuard that must be kept alive for the file writer to flush properly.
pub fn init_logging(gui_sender: Sender<(String, tracing::Level)>) -> anyhow::Result<LoggingGuard> {
    let log_dir = CONFIG_DIR.join("logs");
    std::fs::create_dir_all(&log_dir)?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "tagmonster.log");
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(atty::is(atty::Stream::Stderr))
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter::EnvFilter::from_default_env());

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter::EnvFilter::from_default_env());

    let gui_layer = GuiLayer::new(gui_sender).with_filter(filter::EnvFilter::from_default_env());

    let subscriber = Registry::default()
        .with(file_layer)
        .with(stderr_layer)
        .with(gui_layer);

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(LoggingGuard::new(guard))
}
