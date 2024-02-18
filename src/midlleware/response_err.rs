use actix_web::body::BoxBody;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
struct ResponseBody{
    result: bool,
    message_err: String
}

#[derive(Debug)]
pub struct ResponseErr{
    pub status_code: StatusCode,
    pub message_err: String
}

impl fmt::Display for ResponseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status_code: {} message_err: {}", self.status_code, self.message_err)
    }
}

impl ResponseError for ResponseErr {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody>{

        let res_body = ResponseBody{
            result: false,
            message_err: self.message_err.to_owned()
        };

        let body = serde_json::to_string(&res_body).unwrap();
        
        return HttpResponse::build(self.status_code())
        .content_type(ContentType::json())
        .body(body);

    }
}