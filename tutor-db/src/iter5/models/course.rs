use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(course: web::Json<CreateCourse>) -> Self {
        Self {
            tutor_id: course.tutor_id,
            course_name: course.course_name.clone(),
            course_description: course.course_description.clone(),
            course_format: course.course_format.clone(),
            course_structure: course.course_format.clone(),
            course_level: course.course_level.clone(),
            course_duration: course.course_duration.clone(),
            course_language: course.course_language.clone(),
            course_price: course.course_price,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(update_course: web::Json<UpdateCourse>) -> Self {
        Self {
            course_name: update_course.course_name.clone(),
            course_description: update_course.course_description.clone(),
            course_format: update_course.course_format.clone(),
            course_structure: update_course.course_structure.clone(),
            course_duration: update_course.course_duration.clone(),
            course_price: update_course.course_price,
            course_language: update_course.course_language.clone(),
            course_level: update_course.course_level.clone(),
        }
    }
}

