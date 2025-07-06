pub(crate) mod healthz;
mod users;

use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings};

pub(crate) fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/user" => users::get_routes_and_docs(settings),
    }
}
