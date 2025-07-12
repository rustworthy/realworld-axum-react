use crate::http::guards::SecurityAddon;
use rocket::fairing::AdHoc;
use utoipa::OpenApi;
use utoipa_scalar::Scalar;
use utoipa_scalar::Servable;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/api", api = crate::http::routes::users::UserApiDocs)
        ),
       modifiers(&SecurityAddon)
    )]
struct ApiDoc;

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Openapi3 Scalar UI Stage", move |rocket| async move {
        rocket.mount("/", Scalar::with_url("/scalar", ApiDoc::openapi()))
    })
}
