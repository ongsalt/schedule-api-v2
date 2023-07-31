use std::env;

use sqlx::{self, postgres::PgPool, Pool, Postgres};

use crate::utils::APISchedule;

#[derive(sqlx::FromRow)]
struct Field {
    name: String,
    #[sqlx(default)]
    code: Option<String>,
    #[sqlx(default)]
    link: Option<String>,
    #[sqlx(default)]
    room: Option<String>,
    #[sqlx(rename = "name")]
    teacher_name: String,
}

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn new() -> Db {
        let pool = PgPool::connect(
            &env::var("DATABASE_URL").expect("No environment variable named DATABASE_URL"),
        )
        .await
        .expect("Can't connect to Database");

        Db { pool }
    }

    pub async fn get_schedule(
        &self,
        for_year: i32,
        for_class: i32,
        day: i32,
        period: i32,
    ) -> Result<APISchedule, &'static str> {
        let res = sqlx::query_as::<_, Field>(
            "
            SELECT \"Subject\".name, \"Subject\".code, \"Subject\".link, \"Schedule\".room, \"Teacher\".name  from \"Schedule\" 
            INNER JOIN \"Subject\" ON \"Schedule\".\"subjectId\" = \"Subject\".id 
            INNER JOIN \"_SubjectToTeacher\" ON  \"Subject\".id=\"_SubjectToTeacher\".\"A\"
            INNER JOIN \"Teacher\" ON  \"_SubjectToTeacher\".\"B\"=\"Teacher\".id
            WHERE 
            \"forYear\" = $1 AND 
            \"forRoom\" = $2 AND
            \"period\" = $3 AND
            \"day\" = $4
        ",
        ).bind(for_year).bind(for_class).bind(period).bind(day)
        .fetch_all(&self.pool).await;

        if let Err(e) = res {
            println!("Something went wrong with DB: {}", e.to_string());
            return Err("Something went wrong with DB");
        }

        let data = res.unwrap();

        if data.len() == 0 {
            Err("Not found")
        } else {
            Ok(APISchedule {
                subject_name: data[0].name.to_owned(),
                subject_code: data[0].code.to_owned(),
                room: data[0].room.to_owned(),
                location: "Current".into(), // Next / curernt
                link: data[0].link.to_owned(),
                teachers: data.iter().map(|it| it.teacher_name.to_owned()).collect(),
                day: u32::try_from(day).unwrap(),
                period: u32::try_from(period).unwrap(),
            })
        }
    }
}
