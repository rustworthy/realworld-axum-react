use realworld_rocket_react::{Config, init_tracing, serve};

#[tokio::main]
async fn main() {
    // We are only using the dotenvy crate's functionality when developing
    // the application. This is primarily needed to enable the `sqlx` online
    // mode, which allows is to check types in the app-to-db communication.
    // Having untracked `.env` is also useful for storing API keys that we
    // do not want to expose, but _do_ want to use, for example, for end-to-end
    // testing using the workstation (say, sending emails using a provider or
    // sending telemetry data to a remote backend).
    //
    // To avoid maintenance overhead, we are keeping `.env` file as minimalistic
    // as possible, with almost all the entries being available for copy-pasting
    // from the `.env.example` (which we are also utilizing for documentation).
    #[cfg(debug_assertions)]
    {
        use dotenvy::dotenv;
        dotenv().ok();
    }

    // --------------------  INITIALIZE TELEMETRY  -----------------------------
    let service_name = env!("CARGO_PKG_NAME");
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();
    init_tracing(service_name, otel_endpoint);

    // --------------------     RUN APPLICATION    -----------------------------
    let config = match Config::try_build() {
        Err(e) => panic!("Failed to build applications's configuration: {:?}", e),
        Ok(config) => config,
    };

    // --------------------     RUN APPLICATION    -----------------------------
    if let Err(e) = serve(config).await {
        panic!("Failed to start application: {:?}", e);
    }
}
