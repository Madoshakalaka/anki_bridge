# AnkiBridge

AnkiBridge is a Rust library that serves as a bridge between your Rust code and the Anki application, leveraging the [AnkiConnect](https://ankiweb.net/shared/info/2055492159) add-on to establish an HTTP connection. This library enables seamless transmission of data and facilitates interaction with Anki through Rust.

## Features

AnkiBridge provides the following features:

- Establishing a connection with Anki through the [AnkiConnect](https://ankiweb.net/shared/info/2055492159) add-on.
- Sending requests to Anki for various actions.
- Retrieving data and statistics from Anki.
- Interacting with cards and decks in Anki.

## Installation

To use AnkiBridge in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
anki_bridge = { version = "0.7", features = ["ureq_blocking"] }
```

Additionally, ensure that you have the Anki application installed on your system and that the [AnkiConnect](https://ankiweb.net/shared/info/2055492159) add-on is installed within Anki.

Please note that Anki must be opened and running on your computer for AnkiBridge to establish a connection successfully.

## Usage

To establish a connection and perform actions with Anki, you can utilize the functions and structs provided by the AnkiBridge library in your Rust code. Here's a basic example:

```rust
use std::collections::HashMap;

use anki_bridge::prelude::*;

fn main() {
    let client = AnkiClient::default();
    let decks: Vec<String> = client.request(DeckNamesRequest {}).unwrap();
    println!("{decks:#?}");
    let deck_stats: HashMap<usize, GetDeckStatsResponse> =
        client.request(GetDeckStatsRequest { decks }).unwrap();
    println!("{deck_stats:#?}");
}
```

### Mocking Data

```rust
use anki_bridge::{mock::*, prelude::*};

let client = MockAnkiClient::<FindCardsRequest, _>::new_mock(|params| {
    Ok(vec![123, params.query.len()])
});
let response = client.request(FindCardsRequest {
    query: "Card Deck Name".to_string(),
});
assert_eq!(
    vec![123, "Card Deck Name".len()],
    response.unwrap()
);
```

## Todo

AnkiBridge is an ongoing project with planned future developments. Here are the upcoming items on the to-do list:

- [x] Card Actions
- [x] Deck Actions
- [X] Graphical Actions
- [ ] Media Actions
- [X] Miscellaneous Actions
- [X] Model Actions
- [ ] Note Actions
- [X] Statistic Actions
- [X] [ureq](https://github.com/algesten/ureq) synchronous HTTP client
- [X] [reqwest](https://github.com/seanmonstar/reqwest) asynchronous HTTP client
- [X] Mockable Client
- [ ] Tests (real json test, mocking is already working)

Contributions to AnkiBridge are welcome. Feel free to contribute by opening issues or submitting pull requests on the [GitLab repository](https://gitlab.com/kerkmann/anki_bridge).

## Changelog

The entire changelog can be found in the [CHANGELOG.md](CHANGELOG.md) file.

## Special Thanks

Thanks to [VaiTon](https://github.com/VaiTon) for having the idea to just implement a trait, instead of writing a function. That helped me to write a mockable client and clean everything up. You can find his implementation of the AnkiConnect bridge on [GitHub](https://github.com/VaiTon/ankituls). :)

## License

AnkiBridge is distributed under the [MIT License](https://opensource.org/licenses/MIT). For more information, see the [LICENSE](https://gitlab.com/kerkmann/anki_bridge/blob/main/LICENSE) file.

## Contact

For any questions or inquiries, please contact the project maintainer at [daniel@kerkmann.dev](mailto:daniel@kerkmann.dev).
