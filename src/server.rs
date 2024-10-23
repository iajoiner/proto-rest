use hello_world::{HelloRequest, HelloReply};
use axum::{
    routing::post,
    http::StatusCode,
    Json, Router,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `POST /hello` goes to `say_hello`
        .route("/hello", post(say_hello));

    // run our app with hyper, listening globally on port 3005
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<HelloRequest>,
) -> (StatusCode, Json<HelloReply>) {
    // insert your application logic here
    let reply = HelloReply {
        message: format!("Hello {}!", payload.name),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(reply))
}
