use diesel::{Queryable, Insertable, Associations, Identifiable};
use serde::{Deserialize, Serialize};

use crate::schema::words;

#[derive(Deserialize, Serialize, Queryable, Associations, Identifiable)]
pub struct Word {
    #[serde(skip_serializing)]
    id: i32,
    #[serde(rename(serialize = "motherTongue"))]
    mother_tongue: String,
    #[serde(rename(serialize = "foreignLanguage"))]
    foreign_language: String,
    #[serde(rename(serialize = "secondReading"))]
    second_reading: Option<String>,
}

#[derive(Deserialize, Serialize, Insertable, Queryable, Clone)]
#[table_name = "words"]
pub struct NewWord {
    #[serde(rename(deserialize = "motherTongue"))]
    pub mother_tongue: String,
    #[serde(rename(deserialize = "foreignLanguage"))]
    pub foreign_language: String,
    #[serde(rename(deserialize = "secondReading"))]
    pub second_reading: Option<String>,
}