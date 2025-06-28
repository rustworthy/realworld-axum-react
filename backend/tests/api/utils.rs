use argon2::password_hash;
use argon2::password_hash::rand_core::RngCore as _;
use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use realworld_rocket_react::Config;
use rocket::local::asynchronous::Client;
use testcontainers_modules::postgres;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner as _;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};

pub struct TestContext {
    #[allow(unused)]
    pub container: ContainerAsync<Postgres>,
    pub client: Client,
}

fn gen_b64_secret_key() -> String {
    let mut secret_bytes = [0; 32];
    password_hash::rand_core::OsRng.fill_bytes(&mut secret_bytes);
    BASE64_STANDARD.encode(secret_bytes)
}

pub(crate) async fn setup(test_name: &'static str) -> TestContext {
    // arrange
    let container = postgres::Postgres::default()
        .with_db_name(test_name)
        .with_tag("17")
        .start()
        .await
        .expect("successfully launched PostgreSQL container");
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("port to has been assigned on the host");
    let database_url = format!(
        "postgres://postgres:postgres@localhost:{}/{}",
        host_port, test_name
    );

    let rocket = realworld_rocket_react::construct_rocket(Some(Config {
        migrate: true,
        database_url,
        allowed_origins: None,
        secret_key: gen_b64_secret_key(),
    }));
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket application");
    TestContext { container, client }
}
