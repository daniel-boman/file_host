#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{data::ToByteUnit, http::Status, post, response, Data};
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject, JsonSchema};
use uuid::Uuid;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index, upload])
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}

#[openapi]
#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[openapi]
#[post("/upload", data = "<file>")]
/// Uploads provided file.  
///
/// **Only accepts images.**
async fn upload(
    mut file: Data<'_>,
) -> Result<Json<FileUpload>, response::status::Custom<Json<ApiResponse>>> {
    let header = file.peek(128usize).await;
    let kind = infer::get(&header).expect("Filetype is unknown");

    if !infer::is_image(&header) {
        return Err(response::status::Custom(
            Status::BadRequest,
            Json::from(ApiResponse {
                message: "This route only accepts images",
            }),
        ));
    }

    let id = Uuid::new_v4();
    let ext = kind.extension();

    let file_upload = FileUpload::new(id.to_string(), ext.to_string());
    let filename = format!("upload/{filename}", filename = file_upload.filename());

    if let Err(err) = file.open(5i32.megabytes()).into_file(filename).await {
        println!("Error uploading file: {:?}", err);
        return Err(response::status::Custom(
            Status::InternalServerError,
            Json::from(ApiResponse {
                message: "Failed to upload file",
            }),
        ));
    };

    Ok(Json::from(file_upload))
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Responder)]
struct ApiResponse {
    message: &'static str,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
struct FileUpload {
    id: String,
    ext: String,
    url: Option<String>,
}

impl FileUpload {
    pub fn new(id: String, ext: String) -> Self {
        let mut file_upload = FileUpload {
            id: id,
            ext: ext,
            url: None,
        };
        file_upload.url = Some(file_upload.url());

        file_upload
    }
    pub fn url(&self) -> String {
        format!(
            "{host}/{filename}",
            host = "127.0.0.1:8000",
            filename = self.filename()
        )
    }
    pub fn filename(&self) -> String {
        format!("{id}.{ext}", id = self.id, ext = self.ext)
    }
}

/*
#[openapi]
#[post("/", data = "<file>")]
async fn upload(mut file: Data<'_>) -> crate::Result<FileUpload> {
    let header = file.peek(128usize).await;
    let kind = infer::get(&header).expect("Filetype is unknown");

    if !infer::is_image(&header) {
        return Err(crate::Err);
    }

    let id = Uuid::new_v4();
    let url = format!("{}/{}\n", "http://localhost:8000", id.to_string());
    let ext = kind.extension();
    //let file_upload = FileUpload { id: id.clone() };

    let filename = format!("upload/{}.", id);

    file.open(5i32.megabytes()).into_file(filename).await?;
    /*
    status::Created::new("http://myservice.com/resource.json")
        .tagged_body("{ 'resource': 'Hello, world!' }"); */

}*/
