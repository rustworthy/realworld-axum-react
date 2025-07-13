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
        let _open_api = &OPENAPI_JSON;
        let ui = docs_ui_path.unwrap_or("/".into());
        rocket
            .mount("/", routes![openapi])
            .mount("/", Scalar::with_url(ui, ApiDoc::openapi()))
    })
}
