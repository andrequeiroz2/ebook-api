use actix_web::{
    post,
    get,
    put,
    web::{Data, Json, Path}, 
    Responder, 
    delete,
    Result
};
use actix_files::NamedFile;
use crate::book::book_model::Book;
use crate::database::repo::MongoDB;
use crate::book::book_filter::RequestFilter;


#[post("/book")]
pub async fn create(db: Data<MongoDB>, book: Json<Book>) -> impl Responder{

    let result = db.create(book.to_owned()).await;

    match result {
        Ok(res) => Ok(res),
        
        Err(err) => Err(err),
    }
}

#[post("/books")]
pub async fn create_many(db: Data<MongoDB>, books: Json<Vec<Book>>) -> impl Responder{

    let result = db.create_many(books.to_vec()).await;

    match result {
        Ok(res) => Ok(res),
        
        Err(err) => Err(err),
    }
}

#[get("/book/{id}")]
pub async fn find_one(db: Data<MongoDB>, path: Path<String>) -> impl Responder{

    let id = path.into_inner();

    let book_result = db.find_one(&id).await;

    match book_result {

        Ok(book) => Ok(book),
        
        Err(err) => Err(err),
    }
}

#[get("/books")]
pub async fn find_all(db: Data<MongoDB>) -> impl Responder{

    let books_result = db.find_all().await;

    match books_result {
        
        Ok(books) => Ok(books),

        Err(err) => Err(err),
    }
}

#[put("/book/{id}")]
pub async fn update_one(db: Data<MongoDB>, path: Path<String>, book: Json<Book>) -> impl Responder{

    let id = path.into_inner();

    let book_response = db.update_one(&id, book.to_owned()).await;

    match book_response {

        Ok(result) => Ok(result),

        Err(err) => Err(err),
    }
}

#[delete("/book/{id}")]
pub async fn delete_one(db: Data<MongoDB>, path: Path<String>) -> impl Responder{

    let id = path.into_inner();

    let result = db.delete_one(&id).await;

    match result {
        
        Ok(res) => Ok(res),

        Err(err) => Err(err),
    }
}

#[get("/book_filter")]
pub async fn book_filter(db: Data<MongoDB>, json: Json<RequestFilter>) -> impl Responder{

   let t = RequestFilter{
        operator_logical: json.operator_logical.clone(),
        condition:json.condition.clone()
   };

   let result = db.book_filter(&t).await;

   match result {
       Ok(res) => Ok(res),

       Err(err) => Err(err), 
   }
    
    
}

#[get("/help")]
pub async fn help()-> Result<NamedFile>{
    let file = NamedFile::open("src/book/ebook_help.pdf");

    match file{
        Ok(f) => Ok(f),

        Err(err) => Err(err)?
    }
}