use std::env;

pub fn get_input() -> &'static str {
    match env::var("TEST") {
        Ok(val) => {
            if val == "1" || val == "true" {
                "input_test.txt"
            } else {
                "input.txt"
            }
        }
        Err(_) => "input.txt",
    }
}
