use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::Http;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::SecurityScheme;

pub(crate) struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("some components to be have been registered");
        components.add_security_scheme(
            "HttpAuthBearerJWT",
            SecurityScheme::Http(
                Http::builder()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JSON web token string in Authorization header"))
                    .build(),
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    tags(
      (name = "Users", description = "User management endpoints."),
      (name = "Profiles", description = "Profiles endpoints."),
      (name = "Articles", description = "Articles and feed endpoints."),
      (name = "Tags", description = "Content tags endpoints."),
    ),
    modifiers(&SecurityAddon)
    )]
pub(crate) struct RootApiDoc;
