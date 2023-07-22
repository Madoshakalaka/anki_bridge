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
#![allow(clippy::module_name_repetitions)]

#[cfg(not(any(
    feature = "reqwest_async",
    feature = "reqwest_blocking",
    feature = "ureq_blocking"
)))]
compile_error!("Please include EXACTLY ONE of the following client features: 'reqwest_async', 'reqwest_blocking' or 'ureq_blocking'");

#[cfg(any(
    all(feature = "reqwest_async", feature = "reqwest_blocking"),
    all(feature = "reqwest_async", feature = "ureq_blocking"),
    all(feature = "reqwest_blocking", feature = "ureq_blocking")
))]
compile_error!("Please include ONLY ONE of the following client features: 'reqwest_async', 'reqwest_blocking' or 'ureq_blocking'");

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

/// Module containing card-related actions for `AnkiConnect`.
pub mod card_actions;
/// Module containing deck-related actions for `AnkiConnect`.
pub mod deck_actions;
/// Module containing graphical-related actions for `AnkiConnect`.
pub mod graphical_actions;
/// Module containing statistic-related actions for `AnkiConnect`.
pub mod miscellaneous_actions;
/// Module containing mockable client which can be used in other projects.
pub mod mock;
/// Module containing every module which could be useful;
pub mod prelude;
/// Module containing statistic-related actions for `AnkiConnect`.
pub mod statistic_actions;

/// Represents the possible errors that can occur during the execution of the `anki_connect_send` function.
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(any(feature = "reqwest_async", feature = "reqwest_blocking"))]
    /// Error indicating a failure in sending the request with `reqwest`.
    #[error("send request with reqwest failed")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "ureq_blocking")]
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

/// A specialized `Result` type used in the context of `AnkiConnect` requests.
///
/// It represents either a successful result of type `R` or an error of type `Error`.
type Result<R> = std::result::Result<R, Error>;

/// Represents a response received from `AnkiConnect` API.
#[derive(Deserialize)]
struct AnkiConnectResponse<R> {
    /// The result of the API call, if any.
    result: Option<R>,

    /// The error message, if an error occurred.
    error: Option<String>,
}

pub struct AnkiClient<'a> {
    pub endpoint: &'a str,

    #[cfg(feature = "ureq_blocking")]
    pub agent: ureq::Agent,

    #[cfg(feature = "reqwest_async")]
    pub client: reqwest::Client,

    #[cfg(feature = "reqwest_blocking")]
    pub client: reqwest::blocking::Client,
}

impl<'a> AnkiClient<'a> {
    #[must_use]
    pub fn new(endpoint: &'a str) -> Self {
        Self {
            endpoint,

            #[cfg(feature = "ureq_blocking")]
            agent: ureq::agent(),

            #[cfg(feature = "reqwest_async")]
            client: reqwest::Client::new(),

            #[cfg(feature = "reqwest_blocking")]
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl<'a> Default for AnkiClient<'a> {
    #[must_use]
    fn default() -> Self {
        Self::new("http://localhost:8765")
    }
}

#[maybe_async::sync_impl]
pub trait AnkiRequestable<Request: AnkiRequest> {
    fn request(&self, params: Request) -> Result<Request::Response>;
}

#[maybe_async::async_impl]
#[async_trait::async_trait]
pub trait AnkiRequestable<Request: AnkiRequest + Send> {
    async fn request(&self, params: Request) -> Result<Request::Response>;
}

pub trait AnkiRequest: std::fmt::Debug + Serialize {
    type Response: Default + DeserializeOwned;

    const ACTION: &'static str;
    const VERSION: u8;

    fn to_json(&self) -> Value {
        if json!(self).is_null() {
            json!({
                "action": Self::ACTION,
                "version": Self::VERSION,
            })
        } else {
            json!({
                "action": Self::ACTION,
                "version": Self::VERSION,
                "params": self,
            })
        }
    }
}

#[maybe_async::sync_impl]
impl<'a, Request: AnkiRequest> AnkiRequestable<Request> for AnkiClient<'a> {
    fn request(&self, params: Request) -> Result<Request::Response> {
        #[cfg(feature = "ureq_blocking")]
        let response: AnkiConnectResponse<Request::Response> = self
            .agent
            .post(self.endpoint)
            .send_json(params.to_json())
            .map_err(|error| Error::Ureq(Box::new(error)))?
            .into_json::<AnkiConnectResponse<Request::Response>>()?;

        #[cfg(feature = "reqwest_blocking")]
        let response: AnkiConnectResponse<Request::Response> = {
            self.client
                .post(self.endpoint)
                .json(&params.to_json())
                .send()
                .map_err(Error::Reqwest)?
                .json::<AnkiConnectResponse<Request::Response>>()
                .map_err(Error::Reqwest)
        }?;

        if let Some(error) = response.error {
            Err(Error::Anki(error))
        } else if let Some(result) = response.result {
            Ok(result)
        } else {
            Ok(Default::default())
        }
    }
}

#[maybe_async::async_impl]
#[async_trait::async_trait]
impl<'a, Request: AnkiRequest + Send + 'a> AnkiRequestable<Request> for AnkiClient<'a> {
    async fn request(&self, params: Request) -> Result<Request::Response> {
        let json = params.to_json();

        #[cfg(feature = "reqwest_async")]
        let response: AnkiConnectResponse<Request::Response> = {
            self.client
                .post(self.endpoint)
                .json(&json)
                .send()
                .await
                .map_err(Error::Reqwest)?
                .json::<AnkiConnectResponse<Request::Response>>()
                .await
                .map_err(Error::Reqwest)
        }?;

        if let Some(error) = response.error {
            Err(Error::Anki(error))
        } else if let Some(result) = response.result {
            Ok(result)
        } else {
            Ok(Default::default())
        }
    }
}
