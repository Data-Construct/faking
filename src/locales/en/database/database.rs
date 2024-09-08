use crate::{data::datetime::unix::unix_ts_gen, utils::seeder::{self, gen_range}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn collation() -> String {
      EN_COLLATION[seeder::gen_range(0..EN_COLLATION_LEN)].to_string()
}

#[wasm_bindgen]
pub fn column() -> String {
      EN_COLUMN[seeder::gen_range(0..EN_COLUMN_LEN)].to_string()
}

#[wasm_bindgen]
pub fn engine() -> String {
      EN_ENGINE[seeder::gen_range(0..EN_ENGINE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn column_type() -> String {
      EN_COLUMN_TYPE[seeder::gen_range(0..EN_COLUMN_TYPE_LEN)].to_string()
}

static EN_COLLATION: [&'static str; 7] = [
    "utf8_unicode_ci",
    "utf8_general_ci",
    "utf8_bin",
    "ascii_bin",
    "ascii_general_ci",
    "cp1250_bin",
    "cp1250_general_ci",
];
static EN_COLLATION_LEN: usize = EN_COLLATION.len();

static EN_COLUMN: [&'static str; 14] = [
    "id",
    "title",
    "name",
    "email",
    "phone",
    "token",
    "group",
    "category",
    "password",
    "comment",
    "avatar",
    "status",
    "createdAt",
    "updatedAt",
];
static EN_COLUMN_LEN: usize = EN_COLUMN.len();

static EN_ENGINE: [&'static str; 6] = [
    "InnoDB",
    "MyISAM",
    "MEMORY",
    "CSV",
    "BLACKHOLE",
    "ARCHIVE"
];
static EN_ENGINE_LEN: usize = EN_ENGINE.len();

static EN_COLUMN_TYPE: [&'static str; 24] = [
    "int",
    "varchar",
    "text",
    "date",
    "datetime",
    "tinyint",
    "time",
    "timestamp",
    "smallint",
    "mediumint",
    "bigint",
    "decimal",
    "float",
    "double",
    "real",
    "bit",
    "boolean",
    "serial",
    "blob",
    "binary",
    "enum",
    "set",
    "geometry",
    "point",
];
static EN_COLUMN_TYPE_LEN: usize = EN_COLUMN_TYPE.len();