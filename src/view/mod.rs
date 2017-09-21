use rocket::response::Responder;
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing, AdHoc};
use rocket::http::Status;

pub mod frame;

pub struct ViewContext;

impl ViewContext {
    pub fn fairing() -> impl Fairing {
        AdHoc::on_attach(|rocket| {
            Ok(rocket.manage(ViewContext{}))
        })
    }
}

pub trait View {
    type Response: Responder<'static>;

    fn render(self, ctx: &ViewContext) -> Self::Response;
}

pub use self::frame::Frame;
