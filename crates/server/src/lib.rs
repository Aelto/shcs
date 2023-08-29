use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use actix_web::web::scope;

mod v1;

pub use storage::StorageError;

pub async fn launch_server(
    port: u16,
    buckets_folder: impl Into<std::path::PathBuf>,
    password: String,
) -> Result<(), storage::StorageError> {
    storage::initialize(buckets_folder, None)?;

    let _ = HttpServer::new(move || {
        let logger = Logger::default();

        App::new().wrap(logger).service(
            scope("/v1")
                .app_data(v1::AppData {
                    password: password.clone(),
                })
                .configure(v1::router),
        )
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await?;

    Ok(())
}
