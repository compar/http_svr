
use actix_web::{HttpResponse, web};

use crate::db_access::{get_course_for_teacher_db, post_new_course,get_course_detail_db};
use crate::errors::MyError;
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
)-> Result<HttpResponse,MyError>{
    println!("Received new course");
    post_new_course(&app_state.db, new_course.into()).await
        .map(|course| HttpResponse::Ok().json(course))
   
}
pub  async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
)-> Result<HttpResponse,MyError>{
    println!("Received get_courses_for_teacher");
    let teacher_id =i32::try_from(params.0).unwrap();
    
    get_course_for_teacher_db(&app_state.db,teacher_id)
        .await
        .map(|courses|  HttpResponse::Ok().json(courses))
    
    
   
}
pub  async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,usize)>,
)-> Result<HttpResponse,MyError>{
    println!("Received get_course_detail");
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    get_course_detail_db(&app_state.db, teacher_id, course_id).await
        .map(|course|HttpResponse::Ok().json(course))
}


#[cfg(test)]
mod test{
    use std::env;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;

    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    use actix_web::{web,};
    use super::{AppState,Course};
    use crate::handlers;
    
    #[ignore]
    #[actix_rt::test]
    async  fn post_course_test(){
        dotenv().ok(); //加载配置，如果在生成环境会失败得到Option
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db:db_pool,
        });
    
        let course = web::Json(Course {
            teacher_id:1,
            name:"Test course".to_string(),
            id:Some(555),
            time:None
        });
    
    
        let resp = handlers::new_course(course,app_state).await.unwrap();
    
        println!("{:#?}",     resp.body());
        assert_eq!(resp.status(),StatusCode::OK);
    
    }

}