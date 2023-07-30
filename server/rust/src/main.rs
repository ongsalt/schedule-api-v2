use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod utils;
mod db;

/*
    Todo
     - /api/schedule/[class]/[target]
     - /api/schedule/[class]/[day]/[period]
*/

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/api/schedule/{class}/{target}")]
async fn get_current(target: web::Path<(String, String)>) -> impl Responder {
    if let Ok((year, class)) = utils::parse_class(&target.0) {
        println!("Get {year}/{class}");
        HttpResponse::Ok().body("Ok")
    } else {
        // println!("Panic");
        HttpResponse::BadRequest().body("Gaythai")
    }
}

#[get("/api/schedule/{class}/{day}/{period}")]
async fn get_select(target: web::Path<(String, u32, u32)>) -> impl Responder {
    if let Ok((year, class)) = utils::parse_class(&target.0) {
        println!("Get {year}/{class}");
        HttpResponse::Ok().body("Ok")
    } else {
        // println!("Panic");
        HttpResponse::BadRequest().body("Gaythai")
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_current)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4574))?
    .run()
    .await
}
