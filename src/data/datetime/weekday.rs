use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn weekday() -> String {
  let weekday = WEEKDAYS[seeder::gen_range(0..WEEKDAYS_LEN)];
  return weekday.full.to_string();
}

#[wasm_bindgen]
pub fn weekday_abbr() -> String {
  let weekday = WEEKDAYS[seeder::gen_range(0..WEEKDAYS_LEN)];
  return weekday.short.to_string();
}

static WEEKDAYS: [&'static Weekday; 7] = [
  &DAY_MON,
  &DAY_TUE,
  &DAY_WED,
  &DAY_THU,
  &DAY_FRI,
  &DAY_SAT,
  &DAY_SUN
];
static WEEKDAYS_LEN: usize = WEEKDAYS.len();

struct Weekday<'a> {
  full: &'a str,
  short: &'a str,
  day_of_week: u8,
  day_of_week_iso8601: u8
}

static DAY_MON: Weekday = Weekday {
  full: "Monday",
  short: "Mon",
  day_of_week: 2,
  day_of_week_iso8601: 1
};

static DAY_TUE: Weekday = Weekday {
  full: "Tuesday",
  short: "Tue",
  day_of_week: 3,
  day_of_week_iso8601: 2
};

static DAY_WED: Weekday = Weekday {
  full: "Wednesday",
  short: "Wed",
  day_of_week: 4,
  day_of_week_iso8601: 3
};

static DAY_THU: Weekday = Weekday {
  full: "Thursday",
  short: "Thu",
  day_of_week: 5,
  day_of_week_iso8601: 4
};

static DAY_FRI: Weekday = Weekday {
  full: "Friday",
  short: "Fri",
  day_of_week: 6,
  day_of_week_iso8601: 5
};

static DAY_SAT: Weekday = Weekday {
  full: "Saturday",
  short: "Sat",
  day_of_week: 7,
  day_of_week_iso8601: 6
};

static DAY_SUN: Weekday = Weekday {
  full: "Sunday",
  short: "Sun",
  day_of_week: 1,
  day_of_week_iso8601: 7
};