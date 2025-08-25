#![allow(clippy::vec_init_then_push)]

use argon2::password_hash;
use argon2::password_hash::rand_core::RngCore as _;
use axum::Router;
use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use realworld_axum_react::{Config, MailerTransport};
use secrecy::SecretString;
use sqlx::PgPool;
use std::time::Duration;
use testcontainers_modules::postgres;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner as _;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use tokio::task::JoinHandle;
use url::Url;
use uuid::Uuid;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[cfg(feature = "browser-test")]
use tower_http::services::ServeDir;

const TESTRUN_SETUP_TIMEOUT: Duration = Duration::from_secs(5);

pub struct TestContext {
    #[allow(unused)]
    pub backend_url: Url,

    #[allow(unused)]
    pub db_pool: PgPool,

    #[allow(unused)]
    pub mailer_server: MockServer,

    #[cfg(feature = "api-test")]
    pub http_client: reqwest::Client,

    #[cfg(feature = "browser-test")]
    pub frontend_url: Url,

    #[cfg(feature = "browser-test")]
    pub client: fantoccini::Client,
}

pub struct TestRunContext {
    pub db_container: ContainerAsync<Postgres>,
    pub db_pool: PgPool,
    pub ctx: TestContext,
    pub backend_handle: JoinHandle<()>,
    #[cfg(feature = "browser-test")]
    pub frontend_handle: JoinHandle<()>,
    #[cfg(feature = "browser-test")]
    pub client: fantoccini::Client,
}

fn gen_b64_secret_key() -> String {
    let mut secret_bytes = [0; 32];
    password_hash::rand_core::OsRng.fill_bytes(&mut secret_bytes);
    BASE64_STANDARD.encode(secret_bytes)
}

async fn serve_on_available_port(app: Router) -> (JoinHandle<()>, Url) {
    // prepare a channel to receive the assigned port from
    let (tx, rx) = tokio::sync::oneshot::channel();
    // launch app on any available port (OS will assign one for us)
    let handle = tokio::spawn(async move {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("port to be available");
        let assigned_addr = listener.local_addr().unwrap();
        tx.send(assigned_addr.port()).unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .expect("turned app to service");
    });
    // wait for the app's port
    let port = tokio::time::timeout(TESTRUN_SETUP_TIMEOUT, rx)
        .await
        .expect("test setup to not have timed out")
        .expect("port to have been received from the channel");
    // we now know the app's address
    let url = format!("http://localhost:{}", port)
        .parse()
        .expect("valid url");

    (handle, url)
}

pub(crate) async fn setup(test_name: &'static str) -> TestRunContext {
    // create a PostgreSQL cluster and a database with the `test_name`; since
    // we are using a dedicated cluster for each test, we could in fact go with
    // any database name as long as the app knows the correct connection string;
    // however, we are giving a database exactly the same name as the test has
    // so that if we were to leave containers behind for debugging purposes it
    // would be easier to relate a container with a test;
    let pg_container = postgres::Postgres::default()
        .with_db_name(test_name)
        .with_tag("17")
        .start()
        .await
        .expect("successfully launched PostgreSQL container");
    let host_port = pg_container
        .get_host_port_ipv4(5432)
        .await
        .expect("port to has been assigned on the host");
    let database_url = format!(
        "postgres://postgres:postgres@localhost:{}/{}",
        host_port, test_name
    );
    let pg_pool = PgPool::connect(&database_url)
        .await
        .expect("creds to be correct and db to be accepting connections already");

    #[allow(unused)]
    let frontend_url: Url = "http://localhost".parse().expect("value url");
    // launch front-end application (if browser test)
    #[cfg(feature = "browser-test")]
    let (fe_handle, fe_url) = serve_on_available_port(
        axum::Router::new().fallback_service(ServeDir::new("../frontend/build")),
    )
    .await;
    #[cfg(feature = "browser-test")]
    let frontend_url = fe_url;

    // create app's configuration for testing purposes, making sure to specify
    // our front-end's domain in allowed origins (if browser test)
    #[allow(unused_mut)]
    let mut allowed_origins = Vec::with_capacity(1);

    #[cfg(feature = "browser-test")]
    allowed_origins.push(frontend_url.to_string());

    // create a mock mailer server, i.e. our local instance of Resend,
    // which just intercepts requests and allows us to inspect them
    let mailer_server = MockServer::start().await;
    Mock::given(path("/emails"))
        .and(method("POST"))
        .respond_with(
            // https://resend.com/docs/api-reference/emails/send-email
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": Uuid::new_v4()})),
        )
        .mount(&mailer_server)
        .await;

    let config = Config {
        migrate: true,
        ip: "127.0.0.1".parse().unwrap(),
        port: 0,
        database_url: SecretString::from(database_url),
        secret_key: SecretString::from(gen_b64_secret_key()),
        // https://developers.cloudflare.com/turnstile/troubleshooting/testing/#dummy-sitekeys-and-secret-keys
        captcha_secret: SecretString::from("1x0000000000000000000000000000000AA"),
        docs_ui_path: Some("/scalar".to_string()),
        frontend_url: frontend_url.clone(),
        allowed_origins,
        mailer_transport: MailerTransport::Http,
        mailer_token: SecretString::from("re_"),
        mailer_endpoint: mailer_server.uri().parse().unwrap(),
        mailer_from: "hello@realworld-axum-react.org".to_string(),
        skip_email_verification: None,
        skip_captcha_verification: None,
    };

    // launch back-end application
    let (be_handle, be_url) = serve_on_available_port(
        realworld_axum_react::api(config)
            .await
            .expect("built app and ran migrations just fine"),
    )
    .await;

    // create fantoccini client that the test function will be
    // using to navigate to get the application in the browser
    #[cfg(feature = "browser-test")]
    let client = browser::init_webdriver_client().await;

    // create an HTTP client to call back-end's endpoints as if those were
    // the calls from a script running in the browser or another back-end service
    #[cfg(feature = "api-test")]
    let http_client = reqwest::Client::new();

    // prepare context that the test function is going to
    // receive as its argument and use to perform test actions
    let ctx = TestContext {
        backend_url: be_url,
        db_pool: pg_pool.clone(),
        mailer_server,
        #[cfg(feature = "browser-test")]
        frontend_url: frontend_url.clone(),
        #[cfg(feature = "browser-test")]
        client: client.clone(),
        #[cfg(feature = "api-test")]
        http_client,
    };
    // prepare the "testrunner" context, that our wrapper will use to move
    // the test context to the actual test function and perform clean-up actions
    // after the test execution, such as stopping the database container, closing
    // the webdriver session, killing our web application
    TestRunContext {
        db_container: pg_container,
        db_pool: pg_pool,
        ctx,
        backend_handle: be_handle,
        #[cfg(feature = "browser-test")]
        frontend_handle: fe_handle,
        #[cfg(feature = "browser-test")]
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
        #[tokio::test]
        async fn $test_fn() {
            // setup
            let testrun_ctx = $crate::utils::setup(stringify!($test_fn)).await;

            // test
            let handle = tokio::spawn(super::$test_fn(testrun_ctx.ctx)).await;

            // teardown
            #[cfg(feature = "browser-test")]
            testrun_ctx.frontend_handle.abort();
            #[cfg(feature = "browser-test")]
            testrun_ctx.client.close().await.ok();

            testrun_ctx.backend_handle.abort();
            testrun_ctx.db_pool.close().await;
            testrun_ctx
                .db_container
                .stop_with_timeout(Some(0))
                .await
                .ok();

            // unwind
            if let Err(e) = handle {
                std::panic::resume_unwind(Box::new(e));
            }
        }
    };
}

#[cfg(feature = "browser-test")]
mod browser {
    pub(super) async fn init_webdriver_client() -> fantoccini::Client {
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
        fantoccini::ClientBuilder::native()
            .capabilities(caps)
            .connect("tcp://localhost:4444")
            .await
            .expect("web driver to be available")
    }
}

pub(crate) mod fake {
    use super::TestContext;
    use serde::Deserialize;
    use serde_json::{Value, json};
    use url::Url;

    #[derive(Debug, Clone, Deserialize)]
    #[allow(unused)]
    pub struct UserDetails {
        pub username: String,
        pub email: String,
        pub password: String,
        pub token: String,
        pub bio: String,
        pub image: Option<Url>,
    }

    pub async fn create_activated_user(ctx: &TestContext) -> UserDetails {
        // register new user
        let url = ctx.backend_url.join("/api/users").unwrap();
        let registration = json!({
            "username": "rob.pike",
            "email": "rob.pike@gmail.com",
            "password": "strongandcomplicated",
            "captcha": "test",
        });
        ctx.http_client
            .post(url)
            .json(&json!({
                "user": registration
            }))
            .send()
            .await
            .expect("request to have succeeded");
        // fine the email verification letter in the inbox
        let email_request = ctx
            .mailer_server
            .received_requests()
            .await
            .expect("requests to have been received")
            .first()
            .expect("letter with OTP to have been sent")
            .body_json::<Value>()
            .expect("JSON payload");
        let html = email_request
            .get("html")
            .expect("'html' field to be present in request payload")
            .as_str()
            .expect("html content to be a string");
        // now, let's parse links out of the letter's content; note there are a few
        // links in the OTP letter (app's homepage, email confirmation page, project's
        // repo); the OTP link goes second
        let finder = linkify::LinkFinder::new();
        let links: Vec<_> = finder.links(html).collect();
        let otp_link: Url = links[1].as_str().parse().expect("value URL");
        let otp = otp_link
            .query_pairs()
            .find(|(key, _)| key == "otp")
            .map(|(_, otp)| otp)
            .expect("OTP as query string parameter");
        // use OTP from the email to cofirm email address
        let token = ctx
            .http_client
            .post(ctx.backend_url.join("/api/users/confirm-email").unwrap())
            .json(&json!({
                "user": {
                    "otp": otp,
                    "captcha": "test",
                }
            }))
            .send()
            .await
            .expect("request to have succeeded")
            .json::<Value>()
            .await
            .expect("user details including fresh JWT")
            .get("user")
            .expect("'user' key in the payload's root")
            .as_object()
            .expect("user object under 'user' key")
            .get("token")
            .expect("token field in the user object")
            .as_str()
            .expect("JWT string")
            .to_owned();
        UserDetails {
            username: "rob.pike".to_string(),
            email: "rob.pike@gmail.com".to_string(),
            password: "strongandcomplicated".to_string(),
            image: None,
            bio: String::default(),
            token,
        }
    }
}
