use actix_web::web;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct  Course{
    pub teacher_id : usize,
    pub id : Option<usize>,
    pub name : String,
    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course{
    fn from(value: Json<Course>) -> Self {
       Course{
           teacher_id: value.teacher_id,
           id: value.id,
           name: value.name.clone(),
           time: value.time,
       }
    }
}