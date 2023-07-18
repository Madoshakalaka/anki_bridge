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
use std::marker::PhantomData;

use crate::{AnkiRequest, AnkiRequestable, Result};

pub struct MockAnkiClient<Request, F>
where
    Request: AnkiRequest + Send,
    F: FnOnce(Request) -> Result<Request::Response> + Send + Sync,
{
    phantom: PhantomData<Request>,
    pub result: F,
}

impl<'a, Request, F> MockAnkiClient<Request, F>
where
    Request: AnkiRequest + Send + 'a,
    F: FnOnce(Request) -> Result<Request::Response> + Send + Sync,
{
    pub fn new_mock(result: F) -> Self {
        Self {
            phantom: PhantomData,
            result,
        }
    }
}

#[maybe_async::sync_impl]
impl<'a, Request, F> AnkiRequestable<Request> for MockAnkiClient<Request, F>
where
    Request: AnkiRequest + Send + Sync + 'a,
    F: FnOnce(Request) -> Result<Request::Response> + Send + Sync + Copy,
{
    fn request(&self, params: Request) -> Result<Request::Response> {
        (self.result)(params)
    }
}

#[maybe_async::async_impl]
#[async_trait::async_trait]
impl<'a, Request, F> AnkiRequestable<Request> for MockAnkiClient<Request, F>
where
    Request: AnkiRequest + Send + Sync + 'a,
    F: FnOnce(Request) -> Result<Request::Response> + Send + Sync + Copy,
{
    async fn request(&self, params: Request) -> Result<Request::Response> {
        (self.result)(params)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::card_actions::find_cards::FindCardsRequest;

    use super::*;

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct TestRequest {
        pub data: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
    pub struct TestResponse {
        pub data: String,
    }

    impl AnkiRequest for TestRequest {
        type Response = TestResponse;

        const ACTION: &'static str = "testEndpoint";
        const VERSION: u8 = 6;
    }

    #[cfg(any(feature = "reqwest_blocking", feature = "ureq_blocking"))]
    #[test]
    fn test_sync_mock_client() {
        let client = MockAnkiClient::<TestRequest, _>::new_mock(|params| {
            Ok(TestResponse {
                data: format!("{}World", params.data),
            })
        });
        let response = client.request(TestRequest {
            data: "Hello".to_string(),
        });
        assert_eq!(String::from("HelloWorld"), response.unwrap().data);
    }

    #[cfg(feature = "reqwest_async")]
    #[tokio::test]
    async fn test_async_mock_client() {
        let client = MockAnkiClient::<TestRequest, _>::new_mock(|params| {
            Ok(TestResponse {
                data: format!("{}World", params.data),
            })
        });
        let response = client
            .request(TestRequest {
                data: "Hello".to_string(),
            })
            .await;
        assert_eq!(String::from("HelloWorld"), response.unwrap().data);
    }

    #[cfg(any(feature = "reqwest_blocking", feature = "ureq_blocking"))]
    #[test]
    fn test_sync_find_cards() {
        let client = MockAnkiClient::<FindCardsRequest, _>::new_mock(|params| {
            Ok(vec![123, 456, 789, params.query.len()])
        });
        let response = client.request(FindCardsRequest {
            query: "Card Deck Name".to_string(),
        });
        assert_eq!(
            vec![123, 456, 789, "Card Deck Name".len()],
            response.unwrap()
        );
    }

    #[cfg(feature = "reqwest_async")]
    #[tokio::test]
    async fn test_async_find_cards() {
        let client = MockAnkiClient::<FindCardsRequest, _>::new_mock(|params| {
            Ok(vec![123, 456, 789, params.query.len()])
        });
        let response = client
            .request(FindCardsRequest {
                query: "Card Deck Name".to_string(),
            })
            .await;
        assert_eq!(
            vec![123, 456, 789, "Card Deck Name".len()],
            response.unwrap()
        );
    }
}
