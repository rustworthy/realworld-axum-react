use std::sync::LazyLock;

use crate::http::guards::SecurityAddon;
use rocket::fairing::AdHoc;
use utoipa::OpenApi;
use utoipa_scalar::Scalar;
use utoipa_scalar::Servable;

static OPENAPI_JSON: LazyLock<&'static str> = LazyLock::new(|| {
    ApiDoc::openapi()
        .to_pretty_json()
        .expect("valid json")
        .leak()
});

// https://guides.scalar.com/scalar/scalar-api-references/themes
// https://github.com/scalar/scalar/blob/4526fd59436c1d81fe435674bbb4135a02554b60/packages/themes/src/presets/solarized.css
static SCALAR_HTML: &'static str = r#"
    <!doctype html>
    <html>
    <head>
        <title>Realworld Rocket React | API Docs</title>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
    </head>
    <body>
        <script data-configuration='{"theme": "solarized"}' id="api-reference" type="application/json">
            $spec
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
fn openapi() -> &'static str {
    &OPENAPI_JSON
}

pub(crate) fn stage(docs_ui_path: Option<String>) -> AdHoc {
    AdHoc::on_ignite("Openapi JSON & Scalar UI Stage", move |rocket| async move {
        // generate pretty-formatted openapi spec, on app's startup
        let _open_api = &*OPENAPI_JSON;
        let ui_path = docs_ui_path.unwrap_or("/".into());
        let ui = Scalar::with_url(ui_path, ApiDoc::openapi()).custom_html(SCALAR_HTML);
        rocket.mount("/", routes![openapi]).mount("/", ui)
    })
}
