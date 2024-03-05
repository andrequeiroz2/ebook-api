mod book;
mod database;
use database::repo::MongoDB;
use actix_web::{web::{Data, JsonConfig}, HttpServer, App, error, HttpResponse,};
use book::book_route::{create, create_many, find_one, find_all, update_one, delete_one, book_filter, help};
use book::book_response::BookGenericResponse;

mod midlleware;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    let db = MongoDB::init().await;

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(
                JsonConfig::default().error_handler(|err, _req|{
                    
                    error::InternalError::from_response(
                        "",
                        HttpResponse::BadRequest()
                            .content_type("application/json")
                            .json(BookGenericResponse{result:false, message:err.to_string()})
                    )
                    .into()
                })
            )
            .service(create)
            .service(create_many)
            .service(find_one)
            .service(find_all)
            .service(update_one)
            .service(delete_one)
            .service(book_filter)
            .service(help)

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
