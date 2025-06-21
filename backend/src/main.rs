use realworld_rocket_react::{construct_rocket, init_tracing};

#[rocket::main]
async fn main() {
    // --------------------  INITIALIZE TELEMETRY  -----------------------------
    let service_name = env!("CARGO_PKG_NAME");
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();
    init_tracing(service_name, otel_endpoint);

    // --------------------     RUN APPLICATION    -----------------------------
    // --------------------   MIGRATING IF NEEDED  -----------------------------
    let migrate = std::env::var("MIGRATE")
        .ok()
        .is_some_and(|value| value == "1");

    if let Err(e) = construct_rocket(migrate).launch().await {
        panic!("Failed to launch rocket app: {:?}", e);
    };
}
