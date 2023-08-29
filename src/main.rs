use shcs::server::launch_server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting server at http://localhost:3000");

    launch_server(3000, "buckets", "pass123".to_owned()).await?;

    Ok(())
}
