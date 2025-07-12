use rocket::{fairing::AdHoc, http::Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

fn cors<S>(allowed_origins: &[S]) -> Cors
where
    S: AsRef<str>,
{
    let allowed_origins = AllowedOrigins::some_regex(allowed_origins);
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Patch, Method::Put]
            .into_iter()
            .map(|v| v.into())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS fairing build successfully")
}

pub fn stage(allowed_origins: Option<Vec<String>>) -> AdHoc {
    AdHoc::on_ignite("CORS Stage", move |rocket| async move {
        match allowed_origins {
            Some(origins) => rocket.attach(cors(&origins)),
            None => rocket,
        }
    })
}
