#[macro_use]
extern crate validator_derive;


mod config;
mod db;
mod errors;
mod handlers;
mod models;
use handlers::app_config;


extern crate serde;
use crate::config::Config;
//use chrono::{DateTime, Utc};
use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Response {
    result: String
}

#[derive(Debug,Serialize, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
    id : String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub pwd : String,
}
#[derive(Debug)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub pass_word: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Inputlogin {
    pub email: String,
    pub pwd : String,
}

 #[actix_web::main]
 async fn main() -> std::io::Result<()> {


    //config
        let config : Config= Config::from_env()
            .expect("error while server configuration");

    //pool (allow connection to be reuse for futures requests)

    let pool = config.db_pool().await.expect("pool error");

    //init the crypto service 

    let crypto_service = config.crypto_service();


     HttpServer::new( move || {
         App::new()
                .wrap(Logger::default())
                .data(pool.clone())
                .data(crypto_service.clone())
                .configure(app_config)
                //.route("/signin", web::post().to(signin))
                // .route("/login", web::post().to(login))
                 //.route("/addition2", web::post().to(addition))
                 //  .servic e(
                 //     web::resource("/addition2").route(
                 //         web::post().to(addition2)))
     })         

     .bind(format!("{}:{}",config.host,config.port))?
     .run()
     .await
 }

//enrolement localhost:8082