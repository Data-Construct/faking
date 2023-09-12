pub use self::data::defaults;

pub use self::data::misc;

pub mod data {
    pub mod defaults {
        pub mod name;
        pub mod types;
        pub mod uuids;
    }
    pub mod misc {
        pub mod animals;
        pub mod starwars;
        pub mod yoda;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
