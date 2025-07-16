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
    // we are only using this to hold a guard, once the test context
    // is dropped, the container will be automatically stopped and removed
    _container: ContainerAsync<Postgres>,
    pub client: Client,
}

fn gen_b64_secret_key() -> String {
    let mut secret_bytes = [0; 32];
    password_hash::rand_core::OsRng.fill_bytes(&mut secret_bytes);
    BASE64_STANDARD.encode(secret_bytes)
}

pub(crate) async fn setup(test_name: &'static str) -> TestContext {
    // create a PostgreSQL cluster and a database with the `test_name`; since
    // we are using a dedicated cluster for each test, we could in fact go with
    // any database name as long as the app knows the correct connection string;
    // however, we are giving a database exactly the same name as the test has
    // so that if we were to leave containers behind for debugging purposes it
    // would be easier to relate a container with a test;
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
    // create a rocket instance for this test
    let rocket = realworld_rocket_react::serve(Some(Config {
        migrate: true,
        database_url,
        secret_key: gen_b64_secret_key(),
        port: 8000,
        ..Default::default()
    }));
    // create a client talking to that rocket instance
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket application");
    TestContext {
        client,
        _container: container,
    }
}
