use std::sync::Mutex;
use actix_web::{HttpResponse, web};
use actix_web::http::StatusCode;
use chrono::Utc;
use crate::modules::Course;
use crate::state::AppState;

pub async  fn  health_check_handler(
    app_state: web::Data<AppState>
) ->  HttpResponse{
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{},{} times", health_check_response, visit_count);
    *visit_count +=1;
    HttpResponse::Ok().json(&response)
}

pub  async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
)-> HttpResponse{
    println!("Received new course");
    let course_count = app_state
        .courses.lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course{
        teacher_id: new_course.teacher_id,
        id: Some(course_count+1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}

#[actix_rt::test]
async  fn post_course_test(){
    let course = web::Json(Course {
        teacher_id:1,
        name:"Test course".to_string(),
        id:None,
        time:None
    });
    let app_state = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let resp = new_course(course,app_state).await;


    assert_eq!(resp.status(),StatusCode::OK);

}