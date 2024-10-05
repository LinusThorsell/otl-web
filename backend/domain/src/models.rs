use crate::schema::blog_posts;
use crate::schema::events;
use crate::schema::users;
use crate::schema::scores;
use crate::schema::tours;

use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = blog_posts)]
pub struct NewBlogPost {
    pub title: String,
    pub body: String,
}


#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Tour {
    pub id: i32,
    pub title: String,
    pub location: String,
    pub description: String,
    pub body: String,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub url: String,
    pub score_count: i32,
    pub image: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tours)]
pub struct NewTour {
    pub title: String,
    pub location: String,
    pub description: String,
    pub body: String,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub url: String,
    pub score_count: i32,
    pub image: String,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Associations)]
#[diesel(belongs_to(Tour))]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub location: String,
    pub description: String,
    pub body: String,
    pub date: chrono::NaiveDateTime,
    pub url: String,
    pub image: String,
    pub tour_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub location: String,
    pub description: String,
    pub body: String,
    pub date: chrono::NaiveDateTime,
    pub url: String,
    pub image: String,
    pub tour_id: Option<i32>,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub pdga: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub pdga: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Event))]
pub struct Score {
    pub id: i32,
    pub score: i32,
    pub divcode: String,
    pub event_id: i32,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = scores)]
pub struct NewScore {
    pub score: i32,
    pub divcode: String,
    pub event_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
#[diesel(table_name = apikeys)]
pub struct ApiKey {
    pub id: i32,
    pub apikey: String,
    pub expires: chrono::NaiveDateTime,
}
