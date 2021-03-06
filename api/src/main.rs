#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod models;
mod schema;

use std::sync::Mutex;

use diesel::{SqliteConnection, Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use models::word::{NewWord, Word};
use rocket::{serde::json::Json, fairing::{Fairing, Info, Kind}, Request, Response, http::{Header, Status}, State};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/word")]
fn get_all_words(connection: &Database) -> Json<Vec<Word>> {
    use schema::words;

    let db = &*connection.lock().unwrap();
    let vec: Vec<Word> = words::table.load(db).unwrap();

    Json::from(vec)
}

#[post("/word", data = "<word>")]
fn new_word(word: Json<NewWord>, connection: &Database) -> Status {
    use schema::words;

    let db = &*connection.lock().unwrap();
    let new_word = word.0.clone();

    let word_vec: Vec<Word> = words::dsl::words.filter(words::dsl::foreign_language.eq(new_word.foreign_language)).filter(words::dsl::mother_tongue.eq(new_word.mother_tongue)).load(db).unwrap();
    
    if word_vec.len() > 0 {
        return Status::Conflict;
    }

    diesel::insert_into(words::table).values(&word.0).execute(db).expect("Could not insert new word");

    Status::Created
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

#[options("/word")]
fn option_word() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let connection = SqliteConnection::establish("vocab.db").unwrap();

    rocket::build()
    .attach(CORS)
    .manage(Mutex::from(connection))
    .mount("/", routes![
        index,
        get_word_from_jisho,
        get_all_words,
        new_word,
        option_word
    ])
}

type Database = State<Mutex<SqliteConnection>>;

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