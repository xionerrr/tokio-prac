use chrono::prelude::*;

pub mod controller;

#[derive(Debug)]
pub struct PaymentStatuses {
    pub data: String,
    pub date: DateTime<Utc>,
}

impl PaymentStatuses {
    pub fn new(data: String) -> Self {
        Self {
            data,
            date: Utc::now(),
        }
    }
}

pub fn aws_module() -> PaymentStatuses {
    PaymentStatuses::new("Any  text".to_string())
}
