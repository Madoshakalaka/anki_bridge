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

use serde::{Deserialize, Serialize};

use crate::AnkiRequest;

/// Parameters for the "requestPermission" action.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequestPermissionRequst;

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionResponse {
    pub permission: Permission,
    pub require_api_key: bool,
    pub version: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Permission {
    #[default]
    Denied,
    Granted,
}

impl AnkiRequest for RequestPermissionRequst {
    type Response = RequestPermissionResponse;

    const ACTION: &'static str = "requestPermission";
    const VERSION: u8 = 6;
}
