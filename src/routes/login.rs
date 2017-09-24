use maud::Markup;
use rocket::Route;
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::Form;
use rocket::response::Redirect;
use view;
use model;
use dbpool::DbConn;
use argon2;
use rand;
use diesel;
use schema;
use chrono::{Utc, Duration};

#[derive(FromForm)]
struct LoginForm {
    uname: String,
    passwd: String
}

#[get("/")]
fn form(frame: view::Frame) -> Markup {
    let form = view::Login {
        ..Default::default()
    }.render();

    frame.content(form.into()).render()
}

#[post("/", data="<form>")]
fn login(form: Form<LoginForm>, db: DbConn, mut cookies: Cookies) -> Result<Redirect, Status> {
    use schema::users::dsl::*;
    use diesel::prelude::*;

    let f = form.get();
    let u = users
        .filter(uname.eq(&f.uname))
        .limit(1)
        .load::<model::User>(&*db)
        .map_err(|_| Status::InternalServerError)?;

    let user = match u.first() {
        None => return Ok(Redirect::to("/login")),
        Some(user) => user
    };

    let pass_match = argon2::verify_encoded(&user.passwd, f.passwd.as_bytes())
        .map_err(|_| Status::InternalServerError)?;

    if !pass_match {
        return Ok(Redirect::to("/login"));
    }
    
    let session_hash = {
        use rand::Rng;
        rand::OsRng::new()
            .map_err(|_| Status::InternalServerError)?
            .gen_ascii_chars()
            .take(10)
            .collect::<String>()
    };

    let expires = (Utc::now() + Duration::weeks(1)).naive_utc();

    let new_session = model::Session {
        id: session_hash.clone(),
        user: user.id,
        expires
    };

    diesel::insert(&new_session)
        .into(schema::sessions::table)
        .execute(&*db)
        .map_err(|_| Status::InternalServerError)?;

    cookies.add_private(Cookie::new("session_id", session_hash));

    Ok(Redirect::to("/"))
}

pub fn routes() -> Vec<Route> {
    routes![form, login]
}
