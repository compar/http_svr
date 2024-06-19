use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
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
#[path= "../db_access.rs"]
mod db_access;

#[actix_rt::main]
async  fn main() ->io::Result<()> {
    dotenv().ok(); //加载配置，如果在生成环境会失败得到Option

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK..".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });
    
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(genral_routes)
            .configure(course_routes)
        
    };
    
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await


}