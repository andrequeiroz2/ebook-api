use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct BookGenericResponse<T> {
    pub result: bool,  
    pub message: T,
}

#[derive(Serialize, Deserialize)]
pub struct BookFindResponse<T>{
    pub result: bool,
    pub data: T
}


