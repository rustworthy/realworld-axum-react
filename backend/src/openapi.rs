use crate::http::guards::SecurityAddon;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use std::sync::LazyLock;
use utoipa::OpenApi;

static OPENAPI_JSON: LazyLock<&'static str> = LazyLock::new(|| {
    ApiDoc::openapi()
        .to_pretty_json()
        .expect("valid json")
        .leak()
});

/// Scalar initial html.
///
/// Using [`solarized`](https://guides.scalar.com/scalar/scalar-api-references/themes)
/// theme and ["suppressing"](https://stackoverflow.com/a/13416784) browser's favicon
/// not found error.
static SCALAR_HTML: &str = r#"
    <!doctype html>
    <html>
    <head>
        <title>Realworld Rocket React | API Docs</title>
        <meta charset="utf-8"/>
        <link rel="icon" href="data:image/png;base64,iVBORw0KGgo=">
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
    </head>
    <body>
        <noscript>
            Scalar requires Javascript to function. Please enable it to browse the documentation.
        </noscript>
        <script 
            id="api-reference" 
            data-configuration='{"theme": "solarized"}' 
            data-url="openapi.json" 
        >
        </script>
        <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
    </body>
    </html>
"#;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/api", api = crate::http::routes::users::UserApiDocs)
        ),
       modifiers(&SecurityAddon)
    )]
struct ApiDoc;

#[get("/openapi.json")]
fn openapi() -> (ContentType, &'static str) {
    (ContentType::JSON, &OPENAPI_JSON)
}

#[get("/")]
fn scalar() -> (ContentType, &'static str) {
    (ContentType::HTML, SCALAR_HTML)
}

pub(crate) fn stage(docs_ui_path: Option<String>) -> AdHoc {
    AdHoc::on_ignite("Openapi JSON & Scalar UI Stage", move |rocket| async move {
        // generate pretty-formatted openapi spec, on app's startup
        let _open_api = &*OPENAPI_JSON;
        let ui_path = docs_ui_path.unwrap_or("/".into());
        rocket
            .mount("/", routes![openapi])
            .mount(ui_path, routes![scalar])
    })
}
