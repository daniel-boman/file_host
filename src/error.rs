use rocket::{http::Status, response::Responder};
use rocket_okapi::{
    okapi::openapi3::Responses, response::OpenApiResponderInner, util::add_schema_response,
    JsonSchema,
};

#[derive(Clone, Debug, JsonSchema)]
pub enum Error {
    InvalidFileType(u16, String),
    InternalError(u16, String),
    NotFound(u16),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'o> {
        let body;
        let status = match self {
            Error::InvalidFileType(status, msg) => {
                body = msg.clone();
                status
            }
            Error::InternalError(status, msg) => {
                body = msg.clone();
                status
            }
            Error::NotFound(status) => {
                body = String::new();
                status
            }
        };
        rocket::response::Response::build()
            .status(Status::from_code(status).unwrap())
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}

impl OpenApiResponderInner for Error {
    fn responses(gen: &mut rocket_okapi::gen::OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<String>();
        add_schema_response(&mut responses, 500, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 501, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 502, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 503, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 504, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 400, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 401, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 403, "text/plain", schema.clone())?;
        add_schema_response(&mut responses, 404, "text/plain", schema.clone())?;

        Ok(responses)
    }
}
