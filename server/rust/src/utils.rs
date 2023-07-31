use chrono::prelude::*;
use chrono_tz::Asia::Bangkok;
use serde::Serialize;

#[derive(Serialize)]
struct Target {
    for_year: u32,
    for_room: u32,
    day: u32,
    period: u32,
}

#[derive(Serialize)]
struct Schedule {
    id: String,
    name: String,
    link: String,
    teachers: Vec<String>,
    code: String,
    schedule: Target,
}

#[derive(Serialize)]
pub struct Period {
    is_in_school_time: bool,
    day: u32,
    period: u32,
}

#[derive(Serialize)]
pub struct APISchedule {
    pub subject_name: String,
    pub subject_code: Option<String>,
    pub room: Option<String>,
    pub location: String, // Next / curernt
    pub link: Option<String>,
    pub teachers: Vec<String>,
    pub day: u32,
    pub period: u32,
}

#[derive(Serialize)]
pub enum APIRespond<T> {
    Ok {
        ok: bool,
        data: T,
    },
    Err{
        ok: bool,
        message: String
    },
}

impl<T> APIRespond<T> {
    pub fn new_error(message: String) -> APIRespond<T> {
        APIRespond::Err { ok: false, message }
    }
    pub fn new_ok(data: T) -> APIRespond<T> {
        APIRespond::Ok { ok: true, data }
    }
}


/*
    m65 65 will fail
    6-5 should give (6, 5)
    in case if no splitter(-) present -> use only one digit

    mhhdcufych6kdjbhjrvjef5 -> 6,5
*/
pub fn parse_class(str: &String) -> Result<(i32, i32), &'static str> {
    let mut is_previous_numberic = false;
    let mut first_digit: Option<i32> = None;
    let mut second_digit: Option<i32> = None;
    let mut temp: i32 = 0;

    println!("{str}");

    for c in str.chars() {
        if c.is_numeric() {
            // println!("{c} is numeric");
            temp *= 10;
            temp += i32::try_from(c.to_digit(10).unwrap()).expect("Integer out of bound");
            is_previous_numberic = true;
        } else {
            if is_previous_numberic {
                if first_digit == None {
                    first_digit = Some(temp);
                    temp = 0;
                } else {
                    second_digit = Some(temp);
                    break;
                }
            }
            is_previous_numberic = false;
        }
    }

    // println!("{}", first_digit.unwrap());

    if is_previous_numberic && second_digit == None {
        second_digit = Some(temp);
        // println!("{}", second_digit.unwrap());
    }

    if first_digit == None || second_digit == None {
        return Err("Invalid class");
    }

    Ok((first_digit.unwrap(), second_digit.unwrap()))
}

pub fn get_current_period() -> Period {
    let utc_time = Utc::now();
    let thai_time = utc_time.with_timezone(&Bangkok);

    // Copied from /web/server/service/schedule.ts::getCurrentPeriod
    let minutes_of_day = thai_time.minute() + thai_time.hour() * 60;
    let alert_times = [510, 560, 610, 660, 710, 760, 810, 860, 910, 960];

    let mut count = 0;
    for alert_time in alert_times {
        if minutes_of_day < alert_time {
            break;
        };
        count += 1;
    }

    Period {
        is_in_school_time: (count <= 9) && (count > 0),
        day: thai_time.day(),
        period: count,
    }
}
