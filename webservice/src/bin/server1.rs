use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub fn genral_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

pub async  fn  health_check_handler() -> impl Responder{
    HttpResponse::Ok().json("Actix Web Service is running!")
}
#[actix_rt::main]
async fn main()->io::Result<()>{
    let app = move || App::new().configure(genral_routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}