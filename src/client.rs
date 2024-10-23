use hello_world::HelloRequest;
use reqwest::Client;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:3005/hello")
        .json(&HelloRequest {
            name: "Tonic".into(),
        }) // Send the request as JSON
        .send()
        .await?;

    println!("RESPONSE: {:?}", response.text().await?);

    Ok(())
}
