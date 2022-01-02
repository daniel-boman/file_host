use rocket::{http::Status, response::Responder};
use rocket_okapi::{
    okapi::openapi3::Responses, response::OpenApiResponderInner, util::add_schema_response,
    JsonSchema,
};

#[derive(Clone, Debug, JsonSchema)]
pub enum Error {
    Custom(u16, String),
    BadRequest(String),
    InternalServerError(String),
    Unauthorized(String),
    NotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Custom(_, _) => f.write_str("Custom"),
            Error::BadRequest(_) => f.write_str("BadRequest"),
            Error::InternalServerError(_) => f.write_str("InternalServerError"),
            Error::Unauthorized(_) => f.write_str("Unauthorized"),
            Error::NotFound(_) => f.write_str("NotFound"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(_, _) => "Custom Error",
            Error::BadRequest(_) => "There was something wrong with the request",
            Error::InternalServerError(_) => "There was an internal server error",
            Error::Unauthorized(_) => "The required credentials were missing",
            Error::NotFound(_) => "The resource could not be found",
        }
    }
}

impl Error {
    pub fn code(&self) -> u16 {
        match *self {
            Error::Custom(code, _) => code,
            Error::BadRequest(_) => Status::BadRequest.code,
            Error::InternalServerError(_) => Status::InternalServerError.code,
            Error::Unauthorized(_) => Status::Unauthorized.code,
            Error::NotFound(_) => Status::NotFound.code,
        }
    }
    pub fn msg(&self) -> String {
        match self {
            Error::Custom(_, msg) => msg.clone(),
            Error::BadRequest(msg) => msg.clone(),
            Error::InternalServerError(msg) => msg.clone(),
            Error::Unauthorized(msg) => msg.clone(),
            Error::NotFound(msg) => msg.clone(),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => {
                Error::NotFound("Could not find the requested resource".to_string())
            }
            _ => Error::InternalServerError(e.to_string()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'o> {
        let status = self.code();
        let body = self.msg();

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
