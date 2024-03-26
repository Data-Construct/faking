use std::ops::Sub;

use crate::utils::seeder;
use chrono::{NaiveDate, Days, Utc, Datelike};

pub fn random_date(start: String, end: String, exclude: Option<Vec<String>>) -> String {
    if start == end {
        println!("Timestamps cannot be the same.");
        return String::from("");
    }

    let start_validate = validate_timestamp(start);
    if start_validate.is_none() {
        return String::from("");
    };
    let start_ts = start_validate.unwrap();
    let start_ymd = NaiveDate::from_ymd_opt(start_ts.0, start_ts.1, start_ts.2).unwrap();

    let end_validate = validate_timestamp(end);
    if end_validate.is_none() {
        return String::from("");
    };
    let end_ts = end_validate.unwrap();
    let end_ymd = NaiveDate::from_ymd_opt(end_ts.0, end_ts.1, end_ts.2).unwrap();

    let ast = end_ymd.sub(start_ymd);
    let rand_days = seeder::gen_range(0..ast.num_days());

    if exclude.is_some() {
        let exclude_list: Vec<String> = exclude.unwrap();
        let mut rand_ymd: Option<String> = None;
        while rand_ymd.is_none() {
            let ymd = start_ymd.checked_add_days(Days::new(seeder::gen_range(0..ast.num_days()) as u64));
            if !exclude_list.contains(&ymd.unwrap().to_string()){
                rand_ymd = Some(ymd.unwrap().to_string());
            }
        }
        return rand_ymd.unwrap();
    }

    let rand_ymd = start_ymd.checked_add_days(Days::new(rand_days as u64));
    if rand_ymd.is_some() {
        return rand_ymd.unwrap().to_string();
    }
    else {
        return String::from("");
    }
}

pub fn random_date_dow(start: String, end: String, day_of_week: String) -> String {
    if day_of_week != String::from("Mon") &&
        day_of_week != String::from("Tue") &&
        day_of_week != String::from("Wed") &&
        day_of_week != String::from("Thu") &&
        day_of_week != String::from("Fri") &&
        day_of_week != String::from("Sat") &&
        day_of_week != String::from("Sun") {
        println!("Invalid day of week provided: {}", day_of_week);
        return String::from("");
    }

    if start == end {
        println!("Timestamps cannot be the same.");
        return String::from("");
    }

    let start_validate = validate_timestamp(start);
    if start_validate.is_none() {
        return String::from("");
    };
    let start_ts = start_validate.unwrap();
    let start_ymd = NaiveDate::from_ymd_opt(start_ts.0, start_ts.1, start_ts.2).unwrap();

    let mut start_dow = start_ymd;

    while start_dow.weekday().to_string() != day_of_week {
        start_dow = start_dow.checked_add_days(Days::new(1)).unwrap();
    }

    let end_validate = validate_timestamp(end);
    if end_validate.is_none() {
        return String::from("");
    };
    let end_ts = end_validate.unwrap();
    let end_ymd = NaiveDate::from_ymd_opt(end_ts.0, end_ts.1, end_ts.2).unwrap();

    if start_dow > end_ymd {
        println!("No date available on {} between {} and {}", day_of_week, start_ymd.to_string(), end_ymd.to_string());
        return String::from("");
    }

    let available_weeks = (end_ymd - start_dow).num_days() / 7;

    if available_weeks == 0 {
        return start_dow.to_string();
    }

    let random_week = seeder::gen_range(0..available_weeks);

    let rand_days = random_week * 7;

    let rand_ymd = start_dow.checked_add_days(Days::new(rand_days as u64));

    if rand_ymd.is_some() {
        return rand_ymd.unwrap().to_string();
    }
    else {
        return String::from("");
    }
}

pub fn validate_timestamp(timestamp: String) -> Option<(i32, u32, u32)> {
    let mut date_parts = timestamp.split("-");
    let first = date_parts.next();
    if first.is_none() {
        println!("Incorrect Format for timestamp: {}", timestamp);
        return None;
    }
    let year_str = first.unwrap();
    let year_validate = year_str.parse::<i32>();
    if year_validate.is_err() {
        println!("Invalid value for year: {}", year_str);
        return None;
    }
    let year = year_validate.unwrap();

    let second = date_parts.next();
    if second.is_none() {
        println!("Incorrect Format for timestamp: {}", timestamp);
        return None;
    }
    let month_str = second.unwrap();
    let month_validate = month_str.parse::<u32>();
    if month_validate.is_err() {
        println!("Invalid value for month: {}", month_str);
        return None;
    }
    let month = month_validate.unwrap();
    if month > 12 {
        println!("Invalid value for month: {}", month_str);
    }

    let third = date_parts.next();
    if third.is_none() {
        println!("Incorrect Format for timestamp: {}", timestamp);
        return None;
    }
    let day_str = third.unwrap();
    let day_validate = day_str.parse::<u32>();
    if day_validate.is_err() {
        println!("Invalid value for day: {}", day_str);
        return None;
    }
    let day = day_validate.unwrap();

    // number of days validation
    let mut max_day = match month{
        1 => 31,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 31 // Should never happpen
    };
    if month == 2 {
        let leap_year = max_day % 4;
        if leap_year == 0 {
            max_day = 29;
        }
    }
    if day > max_day {
        println!("Invalid value for day {}", day_str);
        return None;
    }

    let extra_value = date_parts.next();
    if extra_value.is_some() {
        println!("Trailing characters found for timestamp: {}", timestamp);
    }

    Some((year, month, day))
}

pub fn random_date_future(num_days: i32) -> String {
    let rand_days = seeder::gen_range(0..num_days);

    let naive_date_time: NaiveDate = Utc::now().checked_add_days(Days::new(rand_days.try_into().unwrap())).unwrap().naive_utc().into();
    naive_date_time.to_string()
}

pub fn random_date_past(num_days: i32) -> String {
    let rand_days = seeder::gen_range(0..num_days);

    let naive_date_time: NaiveDate = Utc::now().checked_sub_days(Days::new(rand_days.try_into().unwrap())).unwrap().naive_utc().into();
    naive_date_time.to_string()
}

//pub fn random_day_of_week