use std::sync::LazyLock;

use crate::http::guards::SecurityAddon;
use rocket::fairing::AdHoc;
use utoipa::OpenApi;
use utoipa_scalar::Scalar;
use utoipa_scalar::Servable;

const OPENAPI_JSON: LazyLock<&'static str> = LazyLock::new(|| {
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
    &*OPENAPI_JSON
}

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Openapi JSON & Scalar UI Stage", move |rocket| async move {
        // generate pretty-formatted openapi spec, on app's startup
        let _open_api = &*OPENAPI_JSON;
        rocket
            .mount("/", Scalar::with_url("/scalar", ApiDoc::openapi()))
            .mount("/", routes![openapi])
    })
}
