use serde::{Serialize, Serializer};
use json::object;

pub struct BackendResponse {
    response: String,
}

impl BackendResponse {
    pub fn ok(result: &str) -> Self {
        BackendResponse {
            response: object! {
            "status" => true,
            "result" => result,
            }.to_string(),
        }
    }

    pub fn err(message: &str) -> Self {
        BackendResponse {
            response: object! {
            "status" => false,
            "message" => message.replace("\n", "<br>"),
            }.to_string(),
        }
    }
}

impl Serialize for BackendResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.response.serialize(serializer)
    }
}