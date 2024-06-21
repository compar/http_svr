use std::fmt::{Debug, Display, Formatter};
use actix_web::{error, Error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug,Serialize)]
pub enum MyError{
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug,Serialize)]
pub  struct MyErrorResponse{
    error_message : String ,
}

impl MyError{
    fn error_response(&self) ->String{
        match self {
            MyError::DBError(msg)=>{
                println!("Database error occurred:{:?}",msg);
                "Database error".into()
            }MyError::ActixError(msg)=>{
                println!("Server error occurred:{:?}",msg);
                "Internal Server error".into()
            }MyError::NotFound(msg)=>{
                println!("Not found error occurred:{:?}",msg);
                msg.into()
            }
        }
    }
}


impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f,"{}",self)
    }
}

impl error::ResponseError for MyError{
    fn status_code(&self) -> StatusCode {
        match self{
            MyError::DBError(_) | MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,

        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            error_message: self.error_response(),
        })
    }
}

impl  From<actix_web::error::Error> for MyError {
    fn from(err: Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
impl  From<sqlx::error::Error> for MyError {
    fn from(err: sqlx::error::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}
