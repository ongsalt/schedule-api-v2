use chrono::prelude::*;
use chrono_tz::Asia::Bangkok;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Target {
    for_year: u32,
    for_room: u32,
    day: u32,
    period: u32,
}

#[derive(Deserialize, Serialize)]
struct Schedule {
    id: String,
    name: String,
    link: String,
    teachers: Vec<String>,
    code: String,
    schedule: Target,
}

#[derive(Deserialize, Serialize)]
pub struct Period {
    is_in_school_time: bool,
    day: u32,
    period: u32,
}

#[derive(Deserialize, Serialize)]
pub struct APISchedule {
    subject_name: String,
    subject_code: Option<String>,
    room: Option<String>,
    location: String, // Next / curernt 
    link: Option<String>,
    teachers: Vec<String>,
    day: u32,
    period: u32,
}

/*
    m65 65 will fail
    6-5 should give (6, 5)
    in case if no splitter(-) present -> use only one digit

    mhhdcufych6kdjbhjrvjef5 -> 6,5
*/
pub fn parse_class(str: &String) -> Result<(u32, u32), &'static str> {
    let mut is_previous_numberic = false;
    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;
    let mut temp: u32 = 0;

    println!("{str}");
    
    for c in str.chars() {
        if c.is_numeric() {
            // println!("{c} is numeric");
            temp *= 10;
            temp += c.to_digit(10).unwrap();
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
