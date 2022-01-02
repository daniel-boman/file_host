use crate::{db::models, error::Error};
use rocket::fs::NamedFile;

use rocket::tokio::{fs::File, io};
use rocket::{
    data::ToByteUnit,
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    Data,
};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};

use uuid::Uuid;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: upload, get_file]
}

#[openapi(tag = "Files")]
#[get("/get?<id>")]
/// Gets file by id
pub async fn get_file(id: String, pool: &rocket::State<sqlx::PgPool>) -> Result<NamedFile, Error> {
    let file = models::File::get_by_id(id, pool).await?;

    let file_path = std::env::var("UPLOAD_PATH").expect("UPLOAD_PATH not set");

    let named_file = NamedFile::open(format!("{}{}", file_path, file.file_name)).await?;

    Ok(named_file)
}

#[openapi(tag = "Files")]
#[post("/upload", data = "<file>")]
/// Uploads provided file.  
///
/// **Only accepts images.**
pub async fn upload(
    pool: &rocket::State<sqlx::PgPool>,
    api_key: crate::api::key::ApiKey,
    mut file: Data<'_>,
) -> Result<Json<FileUpload>, Error> {
    let header = file.peek(128usize).await;

    if !infer::is_image(&header) {
        return Err(Error::Custom(
            Status::BadRequest.code,
            "This route only accepts images".to_string(),
        ));
    }

    let kind = infer::get(&header).expect("Could not determine filetype");

    let ext = kind.extension().to_string();

    let file_id = Uuid::new_v4().to_string().replace("-", "");

    let file_name = format!("{}.{}", file_id, ext);

    let result = file.open(1u32.gibibytes()).into_bytes().await?;
    if !result.is_complete() {
        return Err(Error::Custom(
            Status::BadRequest.code,
            "File is too big. Max size is 1GiB.".to_string(),
        ));
    }

    let hash = blake3::hash(&result.value);

    let id = models::File::get_id_by_hash(hash.to_string(), pool).await?;
    if let Some(id) = id {
        return Err(Error::Custom(
            Status::BadRequest.code,
            format!("This file already exists with an id of {}", id),
        ));
    }

    let mut file = File::create(format!("upload/{}", file_name)).await?;

    io::copy(&mut result.value.as_slice(), &mut file).await?;

    let id = models::File {
        id: file_id, // gets ignored anyway
        file_name,
        file_hash: hash.to_string(),
        file_type: 0,
        file_size: result.len() as i32,
        uploader: api_key.0,
        upload_date: chrono::Utc::now().naive_utc(),
    }
    .create(pool)
    .await?;

    Ok(Json::from(FileUpload { id, ext }))
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct FileUpload {
    id: String,
    ext: String,
}

/*let header = file.peek(128usize).await;
let kind = infer::get(&header).expect("Filetype is unknown");

if !infer::is_image(&header) {
    return Err(Error::Custom(
        Status::BadRequest.code,
        "This route only accepts images".to_string(),
    ));
}

let id = Uuid::new_v4();
let ext = kind.extension();

let file_upload = FileUpload::new(id.to_string(), ext.to_string());
let filename = format!("upload/{filename}", filename = file_upload.filename());

if let Err(err) = file.open(5i32.megabytes()).into_file(filename).await {
    println!("Error uploading file: {:?}", err);

    return Err(Error::Custom(
        Status::InternalServerError.code,
        "Failed to upload file".to_string(),
    ));
};

Ok(Json::from(file_upload))*/
