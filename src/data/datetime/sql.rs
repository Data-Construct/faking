// NAIVE date datetime timestamp generation for sql targets

/*
https://learn.microsoft.com/en-us/sql/t-sql/functions/date-and-time-data-types-and-functions-transact-sql?view=sql-server-ver16

Data type	Format	Range	Accuracy	Storage size (bytes)	User-defined fractional second precision	Time zone offset
time	hh:mm:ss[.nnnnnnn]	00:00:00.0000000 through 23:59:59.9999999	100 nanoseconds	3 to 5	Yes	No
date	YYYY-MM-DD	0001-01-01 through 9999-12-31	1 day	3	No	No
smalldatetime	YYYY-MM-DD hh:mm:ss	1900-01-01 through 2079-06-06	1 minute	4	No	No
datetime	YYYY-MM-DD hh:mm:ss[.nnn]	1753-01-01 through 9999-12-31	0.00333 second	8	No	No

datetime2	YYYY-MM-DD hh:mm:ss[.nnn nnnn]	0001-01-01 00:00:00.0000000 through 9999-12-31 23:59:59.9999999	100 nanoseconds	6 to 8	Yes	No
datetimeoffset	YYYY-MM-DD hh:mm:ss[.nnn nnnn] [+|-]hh:mm	0001-01-01 00:00:00.0000000 through 9999-12-31 23:59:59.9999999 (in UTC)
 */

use crate::utils::seeder;
use wasm_bindgen::prelude::*;

// TEMP
pub const YEAR_MIN: i32 = 1971;
pub const YEAR_MAX: i32 = 2037;

pub const MONTH_MIN: i32 = 1;
pub const MONTH_MAX: i32 = 12;

// NOTE(clearfeld): TEMP use chrono to get days in month dont use 28 limit
pub const DAYS_MIN: i32 = 1;
pub const DAYS_MAX: i32 = 28;

pub const HOURS_MIN: i32 = 0;
pub const HOURS_MAX: i32 = 23;

pub const MINUTES_MIN: i32 = 0;
pub const MINUTES_MAX: i32 = 59;

pub const SECONDS_MIN: i32 = 0;
pub const SECONDS_MAX: i32 = 59;

pub const NANO_MIN: i32 = 0;
pub const NANO_MAX: i32 = 23;

// SQL Server

// PostgreSQL and MySQL
// MySQL - TIMESTAMP has a range of '1970-01-01 00:00:01' UTC to '2038-01-19 03:14:07' UTC.
// MySQL - DATETIME supported range is '1000-01-01 00:00:00' to '9999-12-31 23:59:59'.

fn zero_pad_string(arg: i32) -> String {
    if arg < 10 {
        format!("0{}",arg)
    } else {
        arg.to_string()
    }
}

#[wasm_bindgen]
pub fn sql_time() -> String {
	format!(
		"{}:{}:{}",
		zero_pad_string(seeder::gen_range(HOURS_MIN..=HOURS_MAX)),
		zero_pad_string(seeder::gen_range(MINUTES_MIN..=MINUTES_MAX)),
		zero_pad_string(seeder::gen_range(SECONDS_MIN..=SECONDS_MAX))
	)
}

#[wasm_bindgen]
pub fn sql_server_time() -> String {
	format!(
		"{}:{}:{}.{}{}{}{}{}{}{}",
		zero_pad_string(seeder::gen_range(HOURS_MIN..=HOURS_MAX)),
		zero_pad_string(seeder::gen_range(MINUTES_MIN..=MINUTES_MAX)),
		zero_pad_string(seeder::gen_range(SECONDS_MIN..=SECONDS_MAX)),

        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9)
	)
}

#[wasm_bindgen]
pub fn sql_date() -> String {
	format!(
		"{}-{}-{}",
		zero_pad_string(seeder::gen_range(YEAR_MIN..=YEAR_MAX)),
		zero_pad_string(seeder::gen_range(MONTH_MIN..=MONTH_MAX)),
		zero_pad_string(seeder::gen_range(DAYS_MIN..=DAYS_MAX))
	)
}

#[wasm_bindgen]
pub fn sql_datetime() -> String {
	format!(
		"{}-{}-{} {}:{}:{}.{}{}{}{}{}",
		zero_pad_string(seeder::gen_range(YEAR_MIN..=YEAR_MAX)),
		zero_pad_string(seeder::gen_range(MONTH_MIN..=MONTH_MAX)),
		zero_pad_string(seeder::gen_range(DAYS_MIN..=DAYS_MAX)),

		zero_pad_string(seeder::gen_range(HOURS_MIN..=HOURS_MAX)),
		zero_pad_string(seeder::gen_range(MINUTES_MIN..=MINUTES_MAX)),
		zero_pad_string(seeder::gen_range(SECONDS_MIN..=SECONDS_MAX)),

        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9)
	)
}

#[wasm_bindgen]
pub fn sql_server_datetime() -> String {
	format!(
		"{}-{}-{} {}:{}:{}.{}{}{}",
		zero_pad_string(seeder::gen_range(YEAR_MIN..=YEAR_MAX)),
		zero_pad_string(seeder::gen_range(MONTH_MIN..=MONTH_MAX)),
		zero_pad_string(seeder::gen_range(DAYS_MIN..=DAYS_MAX)),

		zero_pad_string(seeder::gen_range(HOURS_MIN..=HOURS_MAX)),
		zero_pad_string(seeder::gen_range(MINUTES_MIN..=MINUTES_MAX)),
		zero_pad_string(seeder::gen_range(SECONDS_MIN..=SECONDS_MAX)),

        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9)
	)
}

#[wasm_bindgen]
pub fn sql_server_datetime2() -> String {
	format!(
		"{}-{}-{} {}:{}:{}.{}{}{}{}{}{}{}",
		zero_pad_string(seeder::gen_range(YEAR_MIN..=YEAR_MAX)),
		zero_pad_string(seeder::gen_range(MONTH_MIN..=MONTH_MAX)),
		zero_pad_string(seeder::gen_range(DAYS_MIN..=DAYS_MAX)),

		zero_pad_string(seeder::gen_range(HOURS_MIN..=HOURS_MAX)),
		zero_pad_string(seeder::gen_range(MINUTES_MIN..=MINUTES_MAX)),
		zero_pad_string(seeder::gen_range(SECONDS_MIN..=SECONDS_MAX)),

        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9),
        seeder::gen_range(0..=9)
	)
}
