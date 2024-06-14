use actix_web::web;
use super::handlers::{health_check_handler, new_course};

pub fn genral_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::scope("/courses"))
        .route("/",web::post().to(new_course));
}