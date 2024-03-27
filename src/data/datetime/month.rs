use crate::utils::seeder;
use chrono::Month;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn month() -> String {
  // let month = MONTHS[seeder::gen_range(0..MONTHS_LEN)];
  // return month.full.to_string();
  let m = get_random_month();
  if m.is_none() {
    return "".to_string();
  }

  return m.unwrap().name().to_string();
}

#[wasm_bindgen]
pub fn month_abbr() -> String {
  // let month = MONTHS[seeder::gen_range(0..MONTHS_LEN)];
  // return month.short.to_string();
  let m = get_random_month();
  if m.is_none() {
    return "".to_string();
  }

  return m.unwrap().name().chars().take(3).collect::<String>();
}

#[wasm_bindgen]
pub fn month_ordinal() -> String {
  (seeder::gen_range(1..=12)).to_string()
}

#[wasm_bindgen]
pub fn month_ordinal0() -> String {
  seeder::gen_range(0..12).to_string()
}

fn get_random_month() -> Option<Month> {
  let i: u8 = seeder::gen_range(1..=12);
  match Month::try_from(i) {
    Ok(m) => return Some(m),
    Err(e) => {
      println!("{}\n", e);
      return None;
    }
  }
}

// static MONTHS: [&'static Month; 12] = [
//   &MONTH_JAN,
//   &MONTH_FEB,
//   &MONTH_MAR,
//   &MONTH_APR,
//   &MONTH_MAY,
//   &MONTH_JUN,
//   &MONTH_JUL,
//   &MONTH_AUG,
//   &MONTH_SEPT,
//   &MONTH_OCT,
//   &MONTH_NOV,
//   &MONTH_DEC
// ];
// static MONTHS_LEN: usize = MONTHS.len();

// struct Month<'a> {
//   full: &'a str,
//   short: &'a str,
//   days: u8,
//   days_leap: u8
// }

// const MONTH_JAN: Month = Month {
//   full: "January",
//   short: "Jan",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_FEB: Month = Month {
//   full: "February",
//   short: "Feb",
//   days: 28,
//   days_leap: 29,
// };

// const MONTH_MAR: Month = Month {
//   full: "March",
//   short: "Mar",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_APR: Month = Month {
//   full: "April",
//   short: "Apr",
//   days: 30,
//   days_leap: 30
// };

// const MONTH_MAY: Month = Month {
//   full: "May",
//   short: "May",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_JUN: Month = Month {
//   full: "June",
//   short: "Jun",
//   days: 30,
//   days_leap: 30
// };

// const MONTH_JUL: Month = Month {
//   full: "July",
//   short: "Jul",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_AUG: Month = Month {
//   full: "August",
//   short: "Aug",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_SEPT: Month = Month {
//   full: "September",
//   short: "Sept",
//   days: 30,
//   days_leap: 30
// };

// const MONTH_OCT: Month = Month {
//   full: "October",
//   short: "Oct",
//   days: 31,
//   days_leap: 31
// };

// const MONTH_NOV: Month = Month {
//   full: "November",
//   short: "Nov",
//   days: 30,
//   days_leap: 30
// };

// const MONTH_DEC: Month = Month {
//   full: "December",
//   short: "Dec",
//   days: 31,
//   days_leap: 31
// };