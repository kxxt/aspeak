use std::env;

use rstest::fixture;

pub struct Creds {
    pub key: String,
    pub region: String,
}

#[fixture]
pub fn creds() -> Creds {
    Creds {
        key: env::var("ASPEAK_TEST_KEY")
            .expect("An azure subscription key is required to be set as ASPEAK_TEST_KEY to run the tests."),
        region: env::var("ASPEAK_TEST_REGION")
            .expect("An azure subscription region is required to be set as ASPEAK_TEST_REGION to run the tests."),
    }
}