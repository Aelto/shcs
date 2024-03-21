use actix_web::http::header::ContentDisposition;
use actix_web::http::header::DispositionParam;
use actix_web::web;
use actix_web::web::delete;
use actix_web::web::get;
use actix_web::web::post;
use actix_web::web::put;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;

use actix_multipart::form::MultipartForm;

mod config;
use config::Config;

mod metadata;
use metadata::Metadata;

mod error;
pub use error::ApiError;

mod bearer_token;
use bearer_token::BearerToken;

mod upload_body;
use upload_body::UploadFileBody;

pub mod sdk;

pub fn router(cfg: &mut web::ServiceConfig) {
  if !Config::enabled().unwrap_or_default() {
    println!("INFO: v1 api disabled");

    return;
  }

  cfg
    .app_data(Data::new(
      Config::from_disk().expect("failure reading v1::Config from disk"),
    ))
    .route("", put().to(upload_file))
    .route(
      "/active/{filename}",
      post().to(replace_file_in_active_bucket),
    )
    .route("/{bucket}/{filename}", post().to(replace_file))
    .route("/{bucket}/{filename}/aliased", get().to(serve_aliased_file))
    .route("/{bucket}/{filename}/metadata", get().to(get_file_metadata))
    .route("/{bucket}/{filename}/alias", get().to(get_file_alias))
    .route("}/{bucket}/{filename}/size", get().to(get_file_size))
    .route(
      "/{bucket}/{filename}/metadata",
      post().to(set_file_metadata),
    )
    .route("/{bucket}/{filename}", delete().to(delete_file))
    .service(actix_files::Files::new(
      "/",
      storage::internal::root().expect("failure getting storage root path"),
    ));
}

async fn upload_file(
  MultipartForm(form): MultipartForm<UploadFileBody>, token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token.authenticate(&config, sdk::Operation::Upload).await?;

  let (metadata, unique_id, tempfile) = form.into_metadata(&storage::internal::active_bucket()?)?;

  let storage_path =
    actix_web::web::block(move || storage::persist_tempfile(&unique_id, tempfile.file, metadata))
      .await??;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Created().body(storage_path))
}

async fn replace_file_in_active_bucket(
  path: Path<String>, MultipartForm(form): MultipartForm<UploadFileBody>, token: BearerToken,
  config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token
    .authenticate(&config, sdk::Operation::ReplaceActive)
    .await?;

  let bucket = storage::internal::active_bucket()?;
  let (metadata, _, tempfile) = form.into_metadata(&bucket)?;
  let item = path.into_inner();

  let storage_path = actix_web::web::block(move || {
    let storage_path = storage::internal::storage_path(&bucket, &item);

    storage::replace_tempfile(&storage_path, tempfile.file, metadata)
  })
  .await??;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Created().body(storage_path))
}

async fn replace_file(
  path: Path<(String, String)>, MultipartForm(form): MultipartForm<UploadFileBody>,
  token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token.authenticate(&config, sdk::Operation::Replace).await?;

  let (bucket, item) = path.into_inner();
  let (metadata, _, tempfile) = form.into_metadata(&bucket)?;

  let storage_path = actix_web::web::block(move || {
    let storage_path = storage::internal::storage_path(&bucket, &item);

    storage::replace_tempfile(&storage_path, tempfile.file, metadata)
  })
  .await??;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Created().body(storage_path))
}

async fn serve_aliased_file(
  path: Path<(String, String)>,
) -> Result<actix_files::NamedFile, ApiError> {
  let (bucket, item) = path.into_inner();
  let storage_path = storage::internal::storage_path(&bucket, &item);

  let (file, path) = storage::read(&storage_path)?;
  let file = actix_files::NamedFile::from_file(file, path)?;

  let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;
  let alias = match metadata.map(|m| m.alias) {
    Some(alias) => alias,
    None => item,
  };

  Ok(
    file
      .use_last_modified(true)
      .set_content_disposition(ContentDisposition {
        disposition: actix_web::http::header::DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(alias)],
      }),
  )
}

async fn set_file_metadata(
  path: Path<(String, String)>, custom: Option<Json<serde_json::Value>>, token: BearerToken,
  config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token
    .authenticate(&config, sdk::Operation::MetadataSet)
    .await?;

  let (bucket, item) = path.into_inner();
  let custom = custom.map(|c| c.into_inner());

  let storage_path = storage::internal::storage_path(&bucket, &item);
  let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;
  let metadata = metadata.map(|m| Metadata { custom, ..m });
  storage::internal::set_metadata(&storage_path, metadata)?;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Ok().finish())
}

async fn get_file_metadata(
  path: Path<(String, String)>, token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token
    .authenticate(&config, sdk::Operation::MetadataGet)
    .await?;

  let (bucket, item) = path.into_inner();
  let storage_path = storage::internal::storage_path(&bucket, &item);
  let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Ok().json(metadata.and_then(|m| m.custom)))
}

async fn get_file_alias(
  path: Path<(String, String)>, token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token
    .authenticate(&config, sdk::Operation::MetadataGet)
    .await?;

  let (bucket, item) = path.into_inner();
  let storage_path = storage::internal::storage_path(&bucket, &item);
  let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Ok().json(metadata.map(|m| m.alias)))
}

async fn get_file_size(
  path: Path<(String, String)>, token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token
    .authenticate(&config, sdk::Operation::MetadataGet)
    .await?;

  let (bucket, item) = path.into_inner();
  let storage_path = storage::internal::storage_path(&bucket, &item);
  let (file, _) = storage::read(&storage_path)?;
  let size = file.metadata()?.len();

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Ok().body(size.to_string()))
}

async fn delete_file(
  path: Path<(String, String)>, token: BearerToken, config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let identifier = token.authenticate(&config, sdk::Operation::Delete).await?;

  let (bucket, item) = path.into_inner();
  let storage_path = storage::internal::storage_path(&bucket, &item);

  storage::remove(&storage_path)?;

  token.complete(&config, identifier).await?;
  Ok(HttpResponse::Ok().finish())
}
