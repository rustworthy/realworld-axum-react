use argon2::password_hash;
use argon2::password_hash::rand_core::RngCore as _;
use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use rocket::fs::FileServer;
use rocket::serde::json::serde_json;
use rocket::tokio::net::TcpListener;
use rocket::tokio::spawn;
use rocket::tokio::task::JoinHandle;
use testcontainers_modules::postgres;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner as _;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};

use realworld_rocket_react::Config;
use realworld_rocket_react::construct_rocket;

pub struct TestContext {
    pub url: String,
    pub client: fantoccini::Client,
}

pub struct TestRunContext {
    pub container: ContainerAsync<Postgres>,
    pub handle: JoinHandle<()>,
    pub ctx: TestContext,
    pub client: fantoccini::Client,
}

fn gen_b64_secret_key() -> String {
    let mut secret_bytes = [0; 32];
    password_hash::rand_core::OsRng.fill_bytes(&mut secret_bytes);
    BASE64_STANDARD.encode(secret_bytes)
}

async fn init_webdriver_client() -> fantoccini::Client {
    let mut chrome_args = Vec::new();
    if std::env::var("HEADLESS").ok().is_some() {
        chrome_args.extend(["--headless", "--disable-gpu", "--disable-dev-shm-usage"]);
    }
    let mut caps = serde_json::map::Map::new();
    caps.insert(
        "goog:chromeOptions".to_string(),
        serde_json::json!({
            "args": chrome_args,
        }),
    );
    // let url = (*CHROMEDRIVER).1.clone();
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect("tcp://localhost:4444")
        .await
        .expect("web driver to be available")
}

pub(crate) async fn setup(test_name: &'static str) -> TestRunContext {
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
    // ask OS for an available port
    let port = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("port assigned by OS")
        .local_addr()
        .unwrap()
        .port();
    // address our front-end is available at
    let url = format!("http://localhost:{}", port);
    // create a rocket instance for this test, mounting
    // file server with the front-end build
    let rocket = construct_rocket(Some(Config {
        migrate: true,
        database_url,
        allowed_origins: None,
        secret_key: gen_b64_secret_key(),
        port,
    }))
    .mount("/", FileServer::from("../frontend/build"));
    // launch rocket application on a dedicated thread
    let handle = spawn(async {
        rocket.launch().await.expect("launched rocket app ok");
    });
    // create fantoccini client that the test function will be
    // using to navigate to get the application in the browser
    let client = init_webdriver_client().await;
    // prepare context that the test function is going to
    // receive as its argument and use to perform test actions
    let ctx = TestContext {
        url,
        client: client.clone(),
    };
    // prepare the "testrunner" context, that our wrapper will use to move
    // the test context to the actual test function and perform clean-up actions
    // after the test execution, such as stopping the database container, closing
    // the webdriver session, killing our rocket application
    TestRunContext {
        container,
        handle,
        ctx,
        client,
    }
}

/// Macro for test setup, execution, and cleanup.
///
/// We are using this marco to try and keep our tests concise, "hiding"
/// setup and cleanup actions, but also guaranteeing them.
///
/// Usage:
/// ```no_run
/// async fn test1(ctx: TestContext) {
///     ctx.client.goto(&ctx.url).await.unwrap();
/// }
///
/// async fn test2(ctx: TestContext) {
///     ctx.client.goto(&ctx.url).await.unwrap();
/// }
///
/// mod tests {
///     async_test!(test1);
///     async_test!(test2);
///     // ...
/// }
/// ```
///
/// Another - and probably more elegant approach - would be to create
/// a procedural macro, while a downside is having this way another crate
/// in the project which needs maintenance and whose logic is still tightly
/// coupled to our concrete e2e test needs.
///
/// Here is an example of that alternative approach:
/// https://github.com/mainmatter/gerust/blob/b02ee562d06ec2dc51be812e4bb044ecca2b5260/blueprint/macros/src/lib.rs.liquid#L85-L116
#[macro_export]
macro_rules! async_test {
    ($test_fn:ident) => {
        #[rocket::async_test]
        async fn $test_fn() {
            // setup
            let testrun_ctx = crate::utils::setup(stringify!($test_fn)).await;

            // test
            let handle = rocket::tokio::spawn(super::$test_fn(testrun_ctx.ctx)).await;

            // teardown
            testrun_ctx.handle.abort();
            testrun_ctx.client.close().await.ok();
            testrun_ctx.container.stop_with_timeout(Some(0)).await.ok();

            // unwind
            if let Err(e) = handle {
                std::panic::resume_unwind(Box::new(e));
            }
        }
    };
}
