use shcs::server::launch_server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenvy::dotenv().expect(".env error");
  let port = dotenvy::var("port")
    .expect("failed to get env port")
    .parse()
    .expect("invalid port number");

  println!("starting server at http://localhost:{port}");

  launch_server(port, "buckets").await?;

  Ok(())
}
