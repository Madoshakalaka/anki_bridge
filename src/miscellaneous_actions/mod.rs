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

/// Gets information about the `AnkiConnect` APIs available. The request supports the following params:
/// - [`scopes`](ApiReflectRequest::scopes) - An array of scopes to get reflection information about. The only currently supported value is `"actions"`.
/// - [`actions`](ApiReflectRequest::actions) - Either `null` or an array of API method names to check for. If the value is `null`, the result will list all of the available API actions. If the value is an array of strings, the result will only contain actions which were in this array.
/// The result will contain a list of which scopes were used and a value for each scope. For example, the `"actions"` scope will contain a `"actions"` property which contains a list of supported action names.
pub mod api_reflect;

/// Exports a given deck in `.apkg` format. Returns [true] if successful or [false] otherwise. The optional property [includeSched] (default is [false]) can be specified to include the cards’ scheduling data.
pub mod export_package;

/// Retrieve the list of profiles.
pub mod get_profiles;

/// Imports a file in `.apkg` format into the collection. Returns [true] if successful or [false] otherwise. Note that the file path is relative to Anki’s collection.media folder, not to the client.
pub mod import_package;

/// Selects the profile specified in request.
pub mod load_profile;

/// Performs multiple actions in one request, returning an array with the response of each action (in the given order).
pub mod multi;

/// Tells anki to reload all data from the database.
pub mod reload_collection;

/// Requests permission to use the API exposed by this plugin. This method does not require the API key, and is the only one that accepts requests from any origin; the other methods only accept requests from trusted origins, which are listed under `webCorsOriginList` in the add-on config. `localhost` is trusted by default.
/// Calling this method from an untrusted origin will display a popup in Anki asking the user whether they want to allow your origin to use the API; calls from trusted origins will return the result without displaying the popup. When denying permission, the user may also choose to ignore further permission requests from that origin. These origins end up in the `ignoreOriginList`, editable via the add-on config.
/// The result always contains the [permission] field, which in turn contains either the string `granted` or `denied`, corresponding to whether your origin is trusted. If your origin is trusted, the fields [`requireApiKey`] ([true] if required) and [version] will also be returned.
/// This should be the first call you make to make sure that your application and Anki-Connect are able to communicate properly with each other. New versions of Anki-Connect are backwards compatible; as long as you are using actions which are available in the reported Anki-Connect version or earlier, everything should work fine.
pub mod request_permission;

/// Synchronizes the local Anki collections with `AnkiWeb`.
pub mod sync;

/// Gets the version of the API exposed by this plugin. Currently versions `1` through `6` are defined.
pub mod version;
