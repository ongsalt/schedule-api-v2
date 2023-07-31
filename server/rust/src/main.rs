use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod controller;
mod db;
mod utils;

type Db = db::Db;
type APIRespond<T> = utils::APIRespond<T>;
type APISchedule = utils::APISchedule;

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
async fn get_current(db: web::Data<Db>, params: web::Path<(String, String)>) -> impl Responder {
    let (class, target) = params.to_owned();
    if let Ok((year, class)) = utils::parse_class(&target) {
        println!("Get {year}/{class}");
        let schedule = db.get_schedule(year, class, 1, 1).await;
        HttpResponse::Ok().body("Ok")
    } else {
        // println!("Panic");
        HttpResponse::BadRequest().body("Gaythai")
    }
}

#[get("/api/schedule/{class}/{day}/{period}")]
async fn get_select(db: web::Data<Db>, params: web::Path<(String, u32, u32)>) -> impl Responder {
    let (target, day, period) = params.to_owned();
    let parsed = utils::parse_class(&target);

    if let Err(message) = parsed {
        return HttpResponse::BadRequest()
            .json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    let (year, class) = parsed.unwrap();
    let schedule = db.get_schedule(year, class, 1, 1).await;
    println!("Get {year}/{class}:{day}-{period}");

    if let Err(message) = schedule {
        return HttpResponse::NotFound().json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    HttpResponse::Ok().json(APIRespond::new_ok(schedule.unwrap()))
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting");
    let database = Db::new().await;

    let res = database.get_schedule(6, 5, 1, 2).await;
    if let Ok(data) = res {
        println!("{}", data.teachers[0]);
        println!("{}", data.teachers[1]);
    } else {
        println!("I fucked up: {}", res.err().unwrap());
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(hello)
            .service(get_current)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4574))?
    .run()
    .await
}
