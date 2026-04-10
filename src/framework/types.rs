use serde::{Serialize};

#[derive(Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}
