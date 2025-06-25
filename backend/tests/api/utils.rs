use realworld_rocket_react::Config;
use rocket::local::asynchronous::Client;
use testcontainers_modules::postgres;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::runners::AsyncRunner as _;

pub struct TestContext {
    #[allow(unused)]
    pub container: ContainerAsync<Postgres>,
    pub client: Client,
}

pub(crate) async fn setup(test_name: &'static str) -> TestContext {
    // arrange
    let container = postgres::Postgres::default()
        .with_db_name(test_name)
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
    }));
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket application");
    TestContext { container, client }
}
