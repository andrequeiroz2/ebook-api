use mongodb::{
    Collection, 
    Client, 
    bson::{oid::ObjectId, doc}, results::{UpdateResult, InsertOneResult, InsertManyResult, DeleteResult},
};
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use crate::book::{book_model::Book, book_filter::RequestFilter};
use crate::midlleware::{
    response_err::ResponseErr, 
    response_ok::ResponseOk
};
use actix_web::http::StatusCode;
use futures::stream::TryStreamExt;

pub struct MongoDB{
    col: Collection<Book>
}

impl MongoDB{

    pub async fn init() -> Self{
        
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            
            Ok(conn) => conn.to_string(),
            
            Err(err)=> format!("connection error: {}", err.to_string()),
            
        };

        let client = Client::with_uri_str(uri).await.unwrap();

        let db = client.database("book_api");

        let col = db.collection("book");

        MongoDB{col}
    }

    pub async fn create(&self, book: Book) -> Result<ResponseOk<InsertOneResult>, ResponseErr>{
        
        let book_doc = Book{
            id: None,
            title: book.title,
            genre: book.genre,
            pages: book.pages,
            author: book.author,
        };

        let book_new = self
            .col
            .insert_one(book_doc, None)
            .await;

        match book_new{
            Ok(res) => Ok(ResponseOk{data: res}),

            Err(err)=> Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,
        }
    }

    pub async fn create_many(&self, books: Vec<Book>) -> Result<ResponseOk<InsertManyResult>, ResponseErr>{
        
        let books_new = self
            .col
            .insert_many(books, None)
            .await;

        match books_new {
            Ok(books) => Ok(ResponseOk{data: books}),

            Err(err) => Err(ResponseErr{ status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string() }),
        } 
    }

    pub async fn find_one(&self, id: &String) -> Result<ResponseOk<Book>, ResponseErr>{
        
        let obj_id = match ObjectId::parse_str(id){

            Ok(obj_id) => obj_id,

            Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,

        };

        let filter = doc! {"_id": obj_id};

        let book = self
            .col.find_one(filter, None)
            .await;
            
        match book {
            Ok(b) => {

                match b {
                    Some(e) => Ok(ResponseOk{data: e}),

                    None => Err(ResponseErr{status_code: StatusCode::NOT_FOUND, message_err: "book not found".to_string()})?    
                }
                
            },

            Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?
        } 
    }

    pub async fn find_all(&self) -> Result<ResponseOk<Vec<Book>>, ResponseErr>{
        
        let book = self
            .col.find(None, None)
            .await;

        match book {

            Ok(cursor) => {
                
                let res_cursor  = cursor.try_collect().await;

                match res_cursor {
                    
                    Ok(res) => Ok(ResponseOk{data: res}),

                    Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()}),
                }
            },

            Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,            
        }
    }

    pub async fn update_one(&self, id: &String, book: Book) -> Result<ResponseOk<UpdateResult>, ResponseErr>{

        let obj_id  = match ObjectId::parse_str(id){
            
            Ok(id)=> id,

            Err(err) =>  Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,
        };


        let filter = doc! {"_id": obj_id};

        let book_update = doc! {
            "$set":
                {
                    "id": book.id,
                    "title": book.title,
                    "genre": book.genre,
                    "pages": book.pages,
                    "author": book.author
                },
        };

        let book_result = self
            .col
            .update_one(filter, book_update, None)
            .await;
        
        match book_result {
            Ok(b) => {
                if b.matched_count != 0 {
                    Ok(ResponseOk{data: b})
                }else{
                    Err(ResponseErr{status_code: StatusCode::NOT_FOUND, message_err: "book not found".to_string()})?
                }
            },

            Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,
        }
    }  

    pub async fn delete_one(&self, id: &String) -> Result<ResponseOk<DeleteResult>, ResponseErr>{
        
        let obj_id  = match ObjectId::parse_str(id){
            
            Ok(id)=> id,

            Err(err) =>  Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,
        };

        let filter = doc! {"_id": obj_id};

        let book = self
            .col.delete_one(filter, None)
            .await;

        match book {

            Ok(res) => Ok(ResponseOk { data: res }),

            Err(err) => Err(ResponseErr { status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})   
        }
    }

    pub async fn book_filter(&self, request_filter: &RequestFilter) -> Result<ResponseOk<Vec<Book>>, ResponseErr>{

        let filter_books  = RequestFilter::builder(request_filter);
        
        let book = self
            .col.find(filter_books, None)
            .await;
            
        match book {

            Ok(cursor) =>{

                let res_cursor  = cursor.try_collect().await;
                
                match res_cursor {
                    
                    Ok(res) => Ok(ResponseOk{data: res}),

                    Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()}),
                }
            },
            
            Err(err) => Err(ResponseErr{status_code: StatusCode::INTERNAL_SERVER_ERROR, message_err: err.to_string()})?,
        }
    }
}