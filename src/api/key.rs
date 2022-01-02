use rocket::{
    http::Status,
    request::{self, FromRequest},
};
use rocket_okapi::{
    okapi::openapi3::{SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
    JsonSchema,
};

#[derive(JsonSchema)]
/// Request guard
pub struct ApiKey(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = &'a str;

    async fn from_request(req: &'a request::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = req.rocket().state::<sqlx::PgPool>().unwrap();

        match req.headers().get_one("X-API-Key") {
            Some(key) => {
                if let Ok(_) = crate::db::models::ApiKey::get_key(key, pool).await {
                    request::Outcome::Success(ApiKey(key.to_owned()))
                } else {
                    request::Outcome::Failure((Status::Unauthorized, "API Key is invalid"))
                }
            }
            None => request::Outcome::Failure((Status::Unauthorized, "Missing `X-API-Key header`")),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKey {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires an API key to access".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "X-API-Key".to_owned(),
                location: "header".to_owned(),
            },
            extensions: rocket_okapi::okapi::openapi3::Object::default(),
        };

        let mut security_req = SecurityRequirement::new();

        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());

        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
