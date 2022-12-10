use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::{model::task::Task, repository::ddb::TaskService};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[derive(Deserialize)]
pub struct TaskCompletionRequest {
    result_file: String,
}

#[derive(Deserialize)]
pub struct SubmitTaskREquest {
    user_id: String,
    task_type: String,
    source_file: String,
}

#[derive(Display, Debug)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
        }
    }
}

#[get("/task/{task_global_id}")]
pub async fn get_task(
    ddb_repo: Data<TaskService>,
    task_identifier: Path<TaskIdentifier>,
) -> Result<Json<Task>, TaskError> {
    let task = ddb_repo.get(task_identifier.into_inner().task_global_id);

    match task {
        Some(task) => Ok(Json(task)),
        None => Err(TaskError::TaskNotFound),
    }
}
