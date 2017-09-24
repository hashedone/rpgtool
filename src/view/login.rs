use maud::{html, Markup};

#[derive(Default)]
pub struct Login {
    pub retry: bool,
    pub uname: Option<String>
}

impl Login {
    pub fn render(self) -> Markup {
        let panel = html! {
            div.panel-heading h3.panel-title "Zaloguj"

            div.panel-body {
                div.form-group div.input-group {
                    span.input-group-addon span.glyphicon.glyphicon-user {}
                    @if let Some(uname) = self.uname {
                        input.form-control type="text" placeholder="login" name="uname" text=(uname);
                    } @else {
                        input.form-control type="text" placeholder="login" name="uname";
                    }
                }

                div.form-group div.input-group {
                    span.input-group-addon span.glyphicon.glyphicon-asterisk {}
                    input.form-control type="password" placeholder="*****" name="passwd";
                }
            }

            div.panel-footer {
                button.btn.btn-primary.pull-right type="submit" { "Zaloguj" }
                div.clearfix {}
            }

        };

        html! {
            div.row div class="col-md-4 col-md-offset-4" {
                form method="POST" action="login" {
                    @if self.retry {
                        div.panel.panel-danger (panel)
                    } @else {
                        div.panel.panel-default (panel)
                    }
                }
            } 
        }
    }
}
