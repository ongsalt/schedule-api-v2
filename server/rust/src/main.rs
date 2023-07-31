use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod db;
mod utils;

type Db = db::Db;
type APIRespond<T> = utils::APIRespond<T>;
type APISchedule = utils::APISchedule;
type Period = utils::Period;

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

    let parsed = utils::parse_class(&class);
    if let Err(message) = parsed {
        return HttpResponse::BadRequest()
            .json(APIRespond::<APISchedule>::new_error(message.into()));
    }
    let (year, class) = parsed.unwrap();

    let target = utils::parse_period(&target);
    if let Err(message) = target {
        return HttpResponse::BadRequest()
            .json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    let Period {day, period, is_in_school_time } = utils::get_current_period();

    if !is_in_school_time {
        return HttpResponse::Ok().json(APIRespond::new_ok(utils::APIRespondNotInSchoolTime::new()));
    }

    let schedule = db.get_schedule(year, class, day, period).await;
    println!("Get {year}/{class}:{day}-{period}");

    if let Err(message) = schedule {
        return HttpResponse::NotFound().json(APIRespond::<APISchedule>::new_error(message.into()));
    }

    HttpResponse::Ok().json(APIRespond::new_ok(schedule.unwrap()))
}

#[get("/api/schedule/{class}/{day}/{period}")]
async fn get_select(db: web::Data<Db>, params: web::Path<(String, u32, u32)>) -> impl Responder {
    println!("Second route triggered");
    let (target, day, period) = params.to_owned();
    let parsed = utils::parse_class(&target);

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
