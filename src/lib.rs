/*
* The MIT License (MIT)
*
* Copyright (c) 2023 Daniél Kerkmann <daniel@kerkmann.dev>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

#[cfg(not(any(feature = "ureq")))]
compile_error!("Please include at least one of the following client features: 'ureq'");

/// Module containing card-related actions for AnkiConnect.
pub mod card_actions;
/// Module containing deck-related actions for AnkiConnect.
pub mod deck_actions;

/// Represents the possible errors that can occur during the execution of the `anki_connect_send` function.
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "ureq")]
    /// Error indicating a failure in sending the request with `ureq`.
    #[error("send request with ureq failed")]
    Ureq(#[from] Box<ureq::Error>),

    /// Error indicating a deserialization error.
    #[error("deserialization error")]
    Serde(#[from] std::io::Error),

    /// Error indicating that Anki returned an unexpected error message.
    #[error("anki returned an unexpected error: {0}")]
    Anki(String),
}

/// A specialized `Result` type used in the context of AnkiConnect requests.
///
/// It represents either a successful result of type `R` or an error of type `Error`.
type Result<R> = std::result::Result<R, Error>;

/// Represents a request to be sent to AnkiConnect API.
#[derive(Debug, Serialize)]
struct AnkiConnectRequest<'a, P: Serialize> {
    /// The action to perform in the AnkiConnect API.
    action: &'a str,

    /// The version of the AnkiConnect API.
    version: usize,

    /// Optional parameters to be included in the request. If `None`, no parameters are sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
}

/// Represents a response received from AnkiConnect API.
#[derive(Debug, Deserialize)]
struct AnkiConnectResponse<R> {
    /// The result of the API call, if any.
    result: Option<R>,

    /// The error message, if an error occurred.
    error: Option<String>,
}

/// Sends a request to AnkiConnect with the specified action and parameters,
/// and returns the deserialized response.
///
/// # Arguments
///
/// * `action` - The action to perform in the AnkiConnect API.
/// * `params` - Optional parameters to be included in the request. If `None`, no parameters are sent.
///
/// # Generic Parameters
///
/// * `P` - Type of the parameters to be serialized and sent with the request.
/// * `R` - Type of the response to be deserialized and returned.
///
/// # Returns
///
/// Returns a `Result` containing the deserialized response `R` if the request was successful,
/// or an `Error` if there was an error in the request or response.
fn anki_connect_send<P: Serialize, R: DeserializeOwned + Default>(
    action: &str,
    params: Option<P>,
) -> Result<R> {
    let data = AnkiConnectRequest {
        action,
        version: 6,
        params,
    };

    #[cfg(feature = "ureq")]
    let response = ureq::post("http://localhost:8765")
        .send_json(data)
        .map_err(|error| Error::Ureq(Box::new(error)))?
        .into_json::<AnkiConnectResponse<R>>()?;

    if let Some(error) = response.error {
        Err(Error::Anki(error))
    } else if let Some(result) = response.result {
        Ok(result)
    } else {
        Ok(R::default())
    }
}
