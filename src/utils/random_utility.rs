use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct RandomUtils;

impl RandomUtils {
    pub fn generate_random_string(length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn generate_random_number(min: i32, max: i32) -> i32 {
        rand::thread_rng().gen_range(min..=max)
    }

    pub fn generate_random_bool() -> bool {
        rand::thread_rng().gen_bool(0.5)
    }

    pub fn generate_random_float(min: f64, max: f64) -> f64 {
        rand::thread_rng().gen_range(min..=max)
    }
}
