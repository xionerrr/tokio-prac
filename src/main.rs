use modules::aws::controller::aws_controller;

mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    let result = modules::aws::aws_module();
    let result = aws_controller(result);

    println!("{:#?}", result);
    Ok(())
}
