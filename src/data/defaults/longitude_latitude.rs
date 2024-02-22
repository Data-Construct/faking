use rand::Rng;

pub fn longitude() -> String {
    let mut rng = rand::thread_rng();
    let mut degrees = rng.gen_range(-90..91);
	let mut dir  = 'N';
	if degrees < 0 {
		dir = 'S';
		degrees = degrees * -1;
	}
	let minutes = rng.gen_range(0..61);
	let seconds: f32 = rng.gen_range(0.0..60.0);

	format!("{}°{}'{:.4}\"{}", degrees, minutes, seconds, dir)
}

pub fn latitude() -> String {
    let mut rng = rand::thread_rng();
    let mut degrees = rng.gen_range(-180..181);
	let mut dir  = 'E';
	if degrees < 0 {
		dir = 'W';
		degrees = degrees * -1;
	}
	let minutes = rng.gen_range(0..61);
	let seconds: f32 = rng.gen_range(0.0..60.0);

	format!("{}°{}'{:.4}\"{}", degrees, minutes, seconds, dir)
}

pub fn coordinates() -> String {
	let lon = longitude();
	let lat = latitude();
	format!("{} {}", lon, lat)
}