use super::PaymentStatuses;

#[derive(Debug)]
pub struct AwsData {
    pub data: String,
    pub date: String,
}

pub fn aws_controller(status: PaymentStatuses) -> AwsData {
    let date = status.date.format("%Y-%m-%d %H:%M:%S").to_string();

    AwsData {
        data: status.data,
        date,
    }
}
