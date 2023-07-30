use std::env;

use sqlx::{postgres::PgPool, Pool, Postgres};

use crate::utils::APISchedule;

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

    // pub async fn get_schedule(
    //     for_year: u32,
    //     for_class: u32,
    //     period: u32,
    //     day: u32
    // ) -> APISchedule {
    //     let res = sqlx::query!(r#"
    //         SELECT
    //     "#)

    //     APISchedule {

    //     }
    // }
}
