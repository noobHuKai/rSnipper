use serde::Serialize;
use serde_json::Value;

const RESPONSE_STATUS_CODE_SUCCESS: u32 = 200;
const RESPONSE_STATUS_CODE_FAILURE: u32 = 500;

#[derive(Serialize)]
pub struct JsonResponse {
    code: u32,
    msg: String,
    data: Value,
}

impl JsonResponse {
    pub fn ok(data: String) -> Self {
        Self::ok_with_string(data)
    }
    pub fn ok_with_string(data: String) -> Self {
        Self {
            code: RESPONSE_STATUS_CODE_SUCCESS,
            msg: String::new(),
            data: Value::String(data),
        }
    }
    pub fn ok_with_value(data: Value) -> Self {
        Self {
            code: RESPONSE_STATUS_CODE_SUCCESS,
            msg: String::new(),
            data,
        }
    }

    pub fn fail(msg: String) -> Self {
        Self {
            code: RESPONSE_STATUS_CODE_FAILURE,
            msg,
            data: Value::Null,
        }
    }
}
