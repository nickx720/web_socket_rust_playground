#[macro_use]
extern crate log;

use actix_web::{
    error::{ BlockingError,ResponseError},
    Error as ActixError,HttpResponse,
};
use derive_more::Display;
use diesel::result::{ DatabaseErrorKind,Error as DBError};
use r2d2::Error as PoolError;
use serde::{ Deserialize,Serialize};

#[derive(Debug,Display,PartialEq)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    PoolError(String),
    BlockingError(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self { 
            Error::BadRequest(error) => { 
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            Error::NotFound(message)=> { 
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            Error::Forbidden => HttpResponse::Forbidden().json::<ErrorResponse>("Forbidden".into()),
            _=> { 
                error!("Internal server error: {:?}",self);
                HttpResponse::InternalServerError().json::<ErrorResponse>("Internal Server Error".into())
            }
        }
    }
    
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ErrorResponse { 
pub errors: Vec<String>,
}

impl From<&str> for ErrorResponse {
 fn from(error:&str)-> Self {
 ErrorResponse {
     errors: vec![error.into()],
 }
 }
}

impl From<&String> for ErrorResponse { 
    fn from(error:&String)-> Self { 
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(error: Vec<String>)-> Self { 
        ErrorResponse {
            errors:error
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
