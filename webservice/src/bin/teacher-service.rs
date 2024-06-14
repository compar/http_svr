use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use crate::routes::{course_routes, genral_routes};
use crate::state::AppState;

#[path="../handlers.rs"]
mod  handlers;
#[path="../routes.rs"]
mod routes;
#[path="../state.rs"]
mod state;
#[path= "../modules.rs"]
mod modules;
#[actix_rt::main]
async  fn main() ->io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK..".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(genral_routes)
            .configure(course_routes)
        
    };
    
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await


}