use actix_web::web;
use crate::handlers::course::{get_course_detail, get_courses_for_teacher, new_course};
use crate::handlers::genneral::health_check_handler;

pub fn genral_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::scope("/courses")
        .route("/",web::post().to(new_course))
        .route("/{teacher_id}",web::get().to(get_courses_for_teacher))
        .route("/{teacher_id}/{course_id}",web::get().to(get_course_detail))
        );
}