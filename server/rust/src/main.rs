use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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
    println!("First route triggered");
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
    println!("Second route triggered");
    let (target, day, period) = params.to_owned();
    let parsed = utils::parse_class(&target);

    let day = i32::try_from(day);
    let period = i32::try_from(period);

    if day.is_err() || period.is_err() {
        return HttpResponse::BadRequest()
            .json(APIRespond::<APISchedule>::new_error("Integer out of bound".into()));
    }

    let day = day.unwrap();
    let period = period.unwrap();

    if let Err(message) = parsed {
        return HttpResponse::BadRequest()
            .json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    let (year, class) = parsed.unwrap();
    let schedule = db.get_schedule(year, class, day, period).await;
    println!("Get {year}/{class}:{day}-{period}");

    if let Err(message) = schedule {
        return HttpResponse::NotFound().json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    HttpResponse::Ok().json(APIRespond::new_ok(schedule.unwrap()))
}

async fn manual_hello() -> impl Responder {
    println!("Third route triggered");
    HttpResponse::Ok().body("Hey there!")
}

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
            .service(get_select)
            .service(get_current)
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 4574))?
    .run()
    .await
}
