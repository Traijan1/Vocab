#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, local::blocking::Client, fairing::{Fairing, Info, Kind}, Request, Response, http::Header};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/jisho?<word>")]
async fn get_word_from_jisho(word: String) -> Json<String> {
    let response: String = reqwest::get(format!("https://jisho.org/api/v1/search/words?keyword={word}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Json::from(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    .mount("/", routes![
        index,
        get_word_from_jisho
    ])
}

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}