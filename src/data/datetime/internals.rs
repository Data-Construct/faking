use chrono::NaiveDate;
use crate::utils::seeder;

pub const YEAR_MIN: i32 = (i32::MIN >> 13) + 1;
pub const YEAR_MAX: i32 = (i32::MAX >> 13) - 1;

pub fn gen_year() -> i32 {
  seeder::gen_range(YEAR_MIN..=YEAR_MAX)
}

const MONTH_MIN: u32 = 0;
const MONTH_MAX: u32 = 11;

pub fn gen_month() -> u32 {
  seeder::gen_range(MONTH_MIN..=MONTH_MAX)
}

pub fn gen_days(year: i32, month: u32) -> u32 {
  let days_max = get_days_in_month(year, month);
  seeder::gen_range(1..=days_max)
}

pub fn get_days_in_month(year: i32, month: u32) -> u32 {
  let nd = NaiveDate::from_ymd(
    match month {
      12 => year + 1,
      _ => year
    },
    match month {
      12 => 1,
      _ => month + 1
    },
    1
  ).signed_duration_since(NaiveDate::from_ymd(year, month, 1)).num_days();

  match u32::try_from(nd) {
      Ok(i) => i,
      Err(e) => panic!("{}", e)
  }
}

const HOUR_MIN: u32 = 0;
const HOUR_MAX: u32 = 23;

pub fn gen_hour() -> u32 {
  seeder::gen_range(HOUR_MIN..=HOUR_MAX)
}

const MIN_SEC_MIN: u32 = 0;
const MIN_SEC_MAX: u32 = 59;

pub fn gen_min_sec() -> u32 {
  seeder::gen_range(MIN_SEC_MIN..=MIN_SEC_MAX)
}

pub fn gen_min() -> u32 {
  gen_min_sec()
}

pub fn gen_sec() -> u32 {
  gen_min_sec()
}

pub fn gen_hms() -> (u32, u32, u32) {
  let h = gen_hour();
  let m = gen_min();
  let s = gen_sec();
  return (h, m, s);
}

const MILLI_MIN: u32 = 0;
const MILLI_MAX: u32 = 999;

pub fn gen_milli() -> u32 {
  seeder::gen_range(MILLI_MIN..=MILLI_MAX)
}

const MICRO_MIN: u32 = 0;
const MICRO_MAX: u32 = 999_999;

pub fn gen_micro() -> u32 {
  seeder::gen_range(MICRO_MIN..=MICRO_MAX)
}

const NANO_MIN: u32 = 0;
const NANO_MAX: u32 = 999_999_999;

pub fn gen_nano() -> u32 {
  seeder::gen_range(NANO_MIN..=NANO_MAX)
}