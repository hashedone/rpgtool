use maud::{html, Markup, PreEscaped};
use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome};

#[derive(Default)]
pub struct Frame {
    pub cookies_confirmed: bool,
    pub content: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Frame {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Frame, ()> {
        let cookies_confirmed = req.cookies().get("cookies-confirmed")
            .and_then(|cc| Some(cc.value() == "true"))
            .unwrap_or(false);

        Outcome::Success(Frame {
            cookies_confirmed,
            content: None
        })
    }
}

impl Frame {
    pub fn content(self, content: String) -> Self {
        let mut res = self;
        res.content = Some(content);
        res
    }

    pub fn render(self) -> Markup {
        html! {
            (PreEscaped("<!DOCTYPE html>"))
            html {
                head {
                    meta charset="utf-8";
                    meta http-equiv="X-UA-Compatible" content="IE=edge";
                    meta name="viewport" content="width=device-width, initial-scale=1";

                    title "RPG Tool"

                    link rel="stylesheet" href="static/bootstrap/css/bootstrap.min.css";
                }

                body {
                    nav.navbar.navbar-inverse.navbar-static-top div.container-fluid {
                        div.navbar-header a.navbar-brand href="/" "RPG Tool"

                        ul.nav.navbar-nav.navbar-right
                            li a href="login" { span.glyphicon.glyphicon-log-in {} " Zaloguj" }
                    }

                    div.container-fluid {
                        @if !self.cookies_confirmed {
                            div.alert.alert-warning.alert-dismissible role="alert" {
                                button.close#cookies-confirm type="button" data-dismiss="alert" aria-label="Close" span aria-hidden="true" (PreEscaped("&times;"))
                                p strong "Informacja o plikach cookies"
                                p "Ta strona wykożystuje pliki cookies w celu zapewnienia maksymalnej wygody kożystania z naszego serwisu.
                                Dalsze przeglądanie tej witryny jest jednoznaczne ze zgodą na wykożystanie plików cookies. Pamiętaj, że
                                zawsze możesz zablokować przechowywanie plików cookies kożystając z ustawień swojej przeglądarki."
                            }
                        }
                        @if let Some(content) = self.content {
                            (PreEscaped(content))
                        }
                    }

                    script src="static/js/jquery-3.2.1.min.js" {}
                    script src="static/bootstrap/js/bootstrap.min.js" {}
                    script src="static/js/js.cookie.js" {}
                    script src="static/js/frame.js" {}
                }
            }
        }
    }
}
