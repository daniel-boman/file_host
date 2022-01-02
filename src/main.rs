#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{okapi::openapi3::OpenApi, rapidoc::*, settings::UrlObject};

pub mod api;
pub mod db;
mod error;
mod files;

#[rocket::main]
async fn main() {
    let launch_result = build_rocket().await.launch().await;
    match launch_result {
        Ok(_) => (),
        Err(err) => println!("Error with rocket server: {}", err),
    }
}

async fn build_rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();

    let mut build = db::init(rocket::build())
        .await
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],

                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let settings = rocket_okapi::settings::OpenApiSettings::default();
    let route_spec = (vec![], custom_spec());
    //let route_spec = (openapi_get_routes![index], custom_spec()); // TODO: fix this
    mount_endpoints_and_merged_docs! {
        build, "/v1".to_owned(), settings,
        "/" => route_spec,
        "/file" => files::get_routes_and_docs(&settings),
    };

    // build.register("/", catchers![not_found_index])
    build
}

fn custom_spec() -> OpenApi {
    use rocket_okapi::okapi::openapi3::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "file host".to_owned(),
            description: Some("an api for uploading and serving files".to_owned()),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "http://127.0.0.1:8000/v1".to_owned(),
            description: Some("localhost".to_owned()),
            ..Default::default()
        }],
        ..Default::default()
    }
}

//#[openapi]
//#[get("/")]
#[catch(404)]
fn not_found_index(req: &rocket::Request) -> Result<String, rocket::response::Redirect> {
    if req.uri().path() == "/" {
        return Err(rocket::response::Redirect::to(uri!("/rapidoc")));
    }

    Ok("404 Not Found".to_string())
}
