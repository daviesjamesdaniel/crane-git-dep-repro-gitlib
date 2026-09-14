use serde::Serialize;

#[derive(Serialize)]
pub struct Thing {
    pub n: u32,
}

pub fn hello() -> &'static str {
    "hello from gitlib"
}
