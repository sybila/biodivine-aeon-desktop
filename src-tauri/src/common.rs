use json::object;
use serde::{Serialize, Serializer};

pub type ErrorMessage = String;
pub type JsonString = String;

pub struct OkResponse {
    response: String,
}

pub struct ErrResponse {
    error: String,
}

impl OkResponse {
    pub fn new(result: &str) -> Self {
        OkResponse {
            response: object! {
            "result" => result,
            }
            .to_string(),
        }
    }
}

impl ErrResponse {
    pub fn new(error_message: &str) -> Self {
        ErrResponse {
            error: object! {
            "message" => error_message.replace('\n', "<br>"),
            }
            .to_string(),
        }
    }
}

impl Serialize for OkResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.response.serialize(serializer)
    }
}

impl Serialize for ErrResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.error.serialize(serializer)
    }
}
