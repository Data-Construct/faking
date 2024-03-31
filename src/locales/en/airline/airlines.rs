use std::collections::HashMap;
lazy_static! {
	pub static ref AIRLINE_HASHMAP: HashMap<usize, Vec<String>> = {
		let mut airline_map = HashMap::new();
		airline_map.insert(
			0,
			vec![
				"Aegean Airlines".to_owned(),
				"A3".to_owned(),
			],
		);
		airline_map
	};
}
