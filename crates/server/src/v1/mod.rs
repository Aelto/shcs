use actix_multipart::form::tempfile::TempFile;

use actix_web::http::header::ContentDisposition;
use actix_web::http::header::DispositionParam;
use actix_web::web;
use actix_web::web::delete;
use actix_web::web::get;
use actix_web::web::post;
use actix_web::web::put;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;

use actix_multipart::form::MultipartForm;

mod metadata;
use metadata::Metadata;

mod error;
use error::ApiError;

mod bearer_password;
use bearer_password::BearerPassword;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("", put().to(upload_file))
        .route("/{bucket}/{filename}", post().to(replace_file))
        .route("/{bucket}/{filename}/aliased", get().to(serve_aliased_file))
        .route("/{bucket}/{filename}/metadata", get().to(get_file_metadata))
        .route(
            "/{bucket}/{filename}/metadata",
            post().to(set_file_metadata),
        )
        .service(actix_files::Files::new("/", "buckets"))
        .route("/{bucket}/{filename}", delete().to(delete_file));
}

pub struct AppData {
    pub password: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadFileBody {
    pub metadata: Option<actix_multipart::form::json::Json<serde_json::Value>>,
    pub file: TempFile,
}

pub async fn upload_file(
    MultipartForm(form): MultipartForm<UploadFileBody>,
    _: BearerPassword,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let tempfile = form.file;
    let user_filename = tempfile.file_name;
    let unique_id = nanoid::nanoid!();

    let metadata = Metadata {
        alias: user_filename.unwrap_or_else(|| unique_id.clone()),
        custom: form.metadata.map(|j| j.0),
    };

    let storage_path = actix_web::web::block(move || {
        storage::persist_tempfile(&unique_id, tempfile.file, metadata)
    })
    .await??;

    return Ok(HttpResponse::Created().json(storage_path));
}

pub async fn replace_file(
    path: Path<(String, String)>,
    MultipartForm(form): MultipartForm<UploadFileBody>,
    _: BearerPassword,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let (bucket, item) = path.into_inner();

    let tempfile = form.file;
    let user_filename = tempfile.file_name;
    let unique_id = nanoid::nanoid!();

    let metadata = Metadata {
        alias: user_filename.unwrap_or_else(|| unique_id.clone()),
        custom: form.metadata.map(|j| j.0),
    };

    let storage_path = actix_web::web::block(move || {
        let storage_path = storage::storage_path(&bucket, &item);

        storage::replace_tempfile(&storage_path, tempfile.file, metadata)
    })
    .await??;

    return Ok(HttpResponse::Created().json(storage_path));
}

pub async fn serve_aliased_file(
    path: Path<(String, String)>,
) -> Result<actix_files::NamedFile, ApiError> {
    let (bucket, item) = path.into_inner();
    let storage_path = storage::storage_path(&bucket, &item);

    let (file, path) = storage::read(&storage_path)?;
    let file = actix_files::NamedFile::from_file(file, path)?;

    let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;
    let alias = match metadata.map(|m| m.alias) {
        Some(alias) => alias,
        None => item,
    };

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(alias)],
        }))
}

pub async fn set_file_metadata(
    path: Path<(String, String)>,
    custom: Option<Json<serde_json::Value>>,
    _: BearerPassword,
) -> Result<HttpResponse, ApiError> {
    let (bucket, item) = path.into_inner();
    let custom = custom.map(|c| c.into_inner());

    let storage_path = storage::storage_path(&bucket, &item);
    let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;
    let metadata = metadata.map(|m| Metadata { custom, ..m });
    storage::set_metadata(&storage_path, metadata)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_file_metadata(
    path: Path<(String, String)>,
    _: BearerPassword,
) -> Result<HttpResponse, ApiError> {
    let (bucket, item) = path.into_inner();
    let storage_path = storage::storage_path(&bucket, &item);
    let metadata: Option<Metadata> = storage::deserialize_metadata(&storage_path)?;

    Ok(HttpResponse::Ok().json(metadata.and_then(|m| m.custom)))
}

pub async fn delete_file(
    path: Path<(String, String)>,
    _: BearerPassword,
) -> Result<HttpResponse, ApiError> {
    let (bucket, item) = path.into_inner();
    let storage_path = storage::storage_path(&bucket, &item);

    storage::remove(&storage_path)?;

    Ok(HttpResponse::Ok().finish())
}