use webapp_test::{create_app_route, establish_connection};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = create_app_route();

    let conn = &mut establish_connection();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
