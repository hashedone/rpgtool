use maud::{html, Markup, PreEscaped};
use super::ViewContext;

pub struct Frame;

impl super::View for Frame {
    type Response = Markup;

    fn render(self, _: &ViewContext) -> Self::Response {
        html! {
            (PreEscaped("<!DOCTYPE html>"))
            head {
                (PreEscaped(r#"<meta charset="utf-8">"#))
                (PreEscaped(r#"<meta http-equiv="X-UA-Compatible" content="IE=edge">"#))
                (PreEscaped(r#"<meta name="viewport" content="width=device-width, initial-scale=1">"#))

                title { "RPG Tool" }
            }

            body {
            }
        }
    }
}
