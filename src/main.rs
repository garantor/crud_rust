// use std::io;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};



#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Server is live and working fine!")
}



#[post("/register")]
async fn register_user() -> impl Responder {
    HttpResponse::Ok().body("hey Register")
}


#[get("/greet/{id}")]
async fn great(id:web::Path<u32>) -> impl Responder {
    format!("Hello woolred {} ", id)  
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let port = 8080;
    // let mut user_input = String:: new();
    // io::stdin().read_line(&mut user_input).expect("failed to readlines ..... ");
    // println!("{} the user input ", user_input);

    // let user_input_1:i64 = user_input.trim().parse().unwrap();

    // println!("{} the input ", user_input_1);


    // let number = {
    //     let c =3;
    //     c + 5
    // };

    println!("Starting server now on {} port .... ", port);

    HttpServer:: new(|| App:: new().service(register_user) .service(home).service(great)).bind(("127.0.0.1", port))?.workers(2).run().await

    


}
