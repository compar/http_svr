use chrono::NaiveDateTime;
use sqlx::PgPool;
use crate::modules::Course;

pub async fn get_course_for_teacher_db(pool: &PgPool, teacher_id:i32)->Vec<Course>{
    let rows = sqlx::query!(
        r#"SELECT id , teacher_id,name,time
        From course
        WHERE teacher_id = $1"#,
        teacher_id
    )
        .fetch_all(pool)
        .await
        .unwrap();

    rows.iter()
        .map(|row|Course{
            id: Some(row.id as usize),
            teacher_id: row.teacher_id.unwrap() as usize,
            name: row.name.clone().unwrap(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
        .collect()
}

pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id:i32) -> Course{
    let row =  sqlx::query!(
        r#"SELECT id,teacher_id,name,time
        FROM course
        WHERE teacher_id = $1 and id =$2"#,
        teacher_id,
        course_id
    )
        .fetch_one(pool)
        .await
        .unwrap();

        Course {
            id: Some(row.id as usize),
            teacher_id:row.teacher_id.unwrap() as usize,
            name : row.name.clone().unwrap(),
            time:  Some(NaiveDateTime::from(row.time.unwrap())),
        }
}


pub  async  fn post_new_course(pool: &PgPool, new_course : Course) ->Course{
    let row =  sqlx::query!(
        r#"INSERT INTO course(id,teacher_id, name) VALUES ($1,$2,$3) RETURNING id, teacher_id, name, time"#,
        new_course.id.unwrap() as i32,
        new_course.teacher_id as i32,
        new_course.name
    )
        .fetch_one(pool)
        .await
        .unwrap();
    Course {
        id: Some(row.id as usize),
        teacher_id: row.id as usize,
        name : row.name.clone().unwrap(),
        time:  Some(NaiveDateTime::from(row.time.unwrap())),
    }
}