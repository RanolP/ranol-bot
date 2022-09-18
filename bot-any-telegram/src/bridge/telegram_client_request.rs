use apid::Call;
use apid_telegram_bot::types::{False, True};
use reqores::{ClientRequest, HttpMethod};
use serde::{Deserialize, Serialize};

/// The response contains a JSON object,
/// which always has a Boolean field 'ok'
/// and may have an optional String field 'description' with a human-readable description of the result.
///
/// If 'ok' equals True,
///     the request was successful and the result of the query can be found in the 'result' field.
///
/// In case of an unsuccessful request, 'ok' equals false
///     and the error is explained in the 'description'.
///     An Integer 'error_code' field is also returned,
///     but its contents are subject to change in the future.
///     Some errors may also have an optional field 'parameters' of the type ResponseParameters,
///     which can help to automatically handle the error.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawTelegramApiResult<T> {
    Ok {
        ok: True,
        result: T,
    },
    Err {
        ok: False,
        description: String,
        error_code: i32,
        // TODO:
        // parameters: Option<ResponseParameters>,
    },
}

impl<T> From<RawTelegramApiResult<T>> for Result<T, TelegramApiError> {
    fn from(result: RawTelegramApiResult<T>) -> Self {
        match result {
            RawTelegramApiResult::Ok { result, .. } => Ok(result),
            RawTelegramApiResult::Err {
                description,
                error_code,
                ..
            } => Err(TelegramApiError {
                description,
                error_code,
            }),
        }
    }
}

#[derive(Deserialize)]
pub struct TelegramApiError {
    description: String,
    error_code: i32,
    // TODO:
    // parameters: Option<ResponseParameters>,
}
pub struct TelegramClientRequest<T>
where
    T: Call + Serialize,
{
    pub url: String,
    pub call: T,
}

impl<T> ClientRequest for TelegramClientRequest<T>
where
    T: Call + Serialize,
{
    type Response = T::Response;

    fn url(&self) -> String {
        self.url.clone()
    }

    fn body(&self) -> Option<String> {
        Some(serde_json::to_string(&self.call).unwrap())
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn deserialize(
        &self,
        response: &dyn reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        let resp: RawTelegramApiResult<T::Response> =
            serde_json::from_slice(response.body()).map_err(|e| e.to_string())?;
        let resp: Result<_, _> = resp.into();
        resp.map_err(|e| e.description)
    }
}
