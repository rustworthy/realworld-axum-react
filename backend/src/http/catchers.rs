use rocket::{http::Header, response::Responder};

#[derive(Responder)]
pub enum Error<T> {
    #[response(status = 401)]
    Unauthorized(T, Header<'static>),
}

#[catch(401)]
pub fn unauthorized() -> Error<()> {
    Error::Unauthorized((), Header::new("WWW-Authenticate", "Token"))
}
