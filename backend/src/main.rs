use realworld_rocket_react::{construct_rocket, telemetry::init_tracing};

#[rocket::main]
async fn main() {
    init_tracing(env!("CARGO_PKG_NAME"));
    if let Err(e) = construct_rocket().launch().await {
        panic!("Failed to launch rocket app: {:?}", e);
    };
}
