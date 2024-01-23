use crate::routes::create_router;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening at {}", listener.local_addr()?);
    axum::serve(listener, create_router()).await?;

    Ok(())
}
