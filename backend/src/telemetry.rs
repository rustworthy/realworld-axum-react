use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_panic::panic_hook;
use tracing_subscriber::{
    Registry, filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init_tracing(service_name: &'static str, otel_endpoint: Option<String>) {
    match otel_endpoint {
        Some(otel_endpoint) => {
            let filter_layer = EnvFilter::try_from_default_env().unwrap_or("info".into());
            let trace_layer = {
                let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
                    .with_batch_exporter(
                        opentelemetry_otlp::SpanExporter::builder()
                            .with_tonic()
                            .build()
                            .expect("successfully initialized trace exporter"),
                    )
                    .with_resource(
                        opentelemetry_sdk::Resource::builder()
                            .with_service_name(service_name)
                            .build(),
                    )
                    .build();
                tracing_opentelemetry::OpenTelemetryLayer::new(provider.tracer("root"))
            };
            let log_layer = {
                let provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
                    .with_batch_exporter(
                        opentelemetry_otlp::LogExporter::builder()
                            .with_tonic()
                            .with_endpoint(&otel_endpoint)
                            .build()
                            .expect("successfully initialized trace exporter"),
                    )
                    .with_resource(
                        opentelemetry_sdk::Resource::builder()
                            .with_service_name(service_name)
                            .build(),
                    )
                    .build();
                opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&provider)
            };
            Registry::default()
                .with(filter_layer)
                .with(JsonStorageLayer)
                .with(BunyanFormattingLayer::new(
                    service_name.into(),
                    std::io::stdout,
                ))
                .with(trace_layer)
                .with(log_layer)
                .init();
        }
        None => {
            let filter_layer = EnvFilter::try_from_default_env().unwrap_or("debug".into());
            Registry::default()
                .with(filter_layer)
                .with(fmt::layer())
                .init();
        }
    }
    std::panic::set_hook(Box::new(panic_hook));
}
