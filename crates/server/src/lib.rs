use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use actix_web::web::get;
use actix_web::web::scope;

pub mod v1;

pub use storage::StorageError;

pub async fn launch_server(
  port: u16, buckets_folder: impl Into<std::path::PathBuf>,
) -> Result<(), storage::StorageError> {
  let buckets_folder: std::path::PathBuf = buckets_folder.into();
  let tempfolder = buckets_folder.clone().join("");

  storage::initialize(buckets_folder, None)?;

  let _ = HttpServer::new(move || {
    let logger = Logger::default();

    App::new()
      .wrap(logger)
      .app_data(
        actix_multipart::form::tempfile::TempFileConfig::default().directory(tempfolder.clone()),
      )
      .route("robots.txt", get().to(robots_txt))
      .service(scope("/v1").configure(v1::router))
  })
  .bind(format!("127.0.0.1:{port}"))?
  .run()
  .await?;

  Ok(())
}

/// This robots.txt disable everything from being indexed
async fn robots_txt() -> &'static str {
  "User-agent: *
Disallow: /"
}
