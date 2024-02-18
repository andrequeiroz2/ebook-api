use actix_web::{Responder, HttpResponse, HttpRequest};
use serde::Serialize;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;

#[derive(Serialize)]
struct ResponseBody<T>
    where      
        T: Serialize,
    {
        result: bool,
        data: T
    }

#[derive(Debug, Serialize)]
pub struct ResponseOk<T>
    where      
        T: Serialize,
    {
        pub data: T,
    }

impl<T: Serialize> Responder for ResponseOk<T>{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        
        let res_body = ResponseBody{
            result: true,
            data: self.data
        };

        let res_body = serde_json::to_string(&res_body).unwrap();
 
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
   }
}