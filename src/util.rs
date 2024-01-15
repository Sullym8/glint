pub fn gen_random() -> f64 {
    rand::random()
}

pub fn gen_random_range(min: f64, max: f64) -> f64 {
    let r = gen_random();
    r * (max - min) + min
}