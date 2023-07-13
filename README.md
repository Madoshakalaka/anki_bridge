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
anki_bridge = { version = "0.4", features = ["ureq_blocking"] }
```

Additionally, ensure that you have the Anki application installed on your system and that the [AnkiConnect](https://ankiweb.net/shared/info/2055492159) add-on is installed within Anki.

Please note that Anki must be opened and running on your computer for AnkiBridge to establish a connection successfully.

## Usage

To establish a connection and perform actions with Anki, you can utilize the functions and structs provided by the AnkiBridge library in your Rust code. Here's a basic example:

```rust
use anki_bridge::deck_actions::{GetDeckStatsParams, get_deck_stats};

fn main() {
    // Establish the parameters for retrieving deck statistics
    let params = GetDeckStatsParams {
        decks: vec!["Deck1".to_string(), "Deck2".to_string()],
    };

    // Retrieve deck statistics from Anki
    match get_deck_stats(params) {
        Ok(deck_stats) => {
            for (deck_name, stats) in deck_stats {
                println!("Deck: {}", deck_name);
                println!("Total Cards: {}", stats.total_in_deck);
                println!("New Cards: {}", stats.new_count);
                println!("Learning Cards: {}", stats.learn_count);
                println!("Review Cards: {}", stats.review_count);
                println!("---");
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
```

## Todo

AnkiBridge is an ongoing project with planned future developments. Here are the upcoming items on the to-do list:

- [x] Card Actions
- [x] Deck Actions
- [X] Graphical Actions
- [ ] Media Actions
- [ ] Miscellaneous Actions
- [ ] Model Actions
- [ ] Note Actions
- [ ] Statistic Actions
- [X] [ureq](https://github.com/algesten/ureq) synchronous HTTP client
- [X] [reqwest](https://github.com/seanmonstar/reqwest) asynchronous HTTP client
- [ ] Tests

Contributions to AnkiBridge are welcome. Feel free to contribute by opening issues or submitting pull requests on the [GitLab repository](https://gitlab.com/kerkmann/anki_bridge).

## License

AnkiBridge is distributed under the [MIT License](https://opensource.org/licenses/MIT). For more information, see the [LICENSE](https://gitlab.com/kerkmann/anki_bridge/blob/main/LICENSE) file.

## Contact

For any questions or inquiries, please contact the project maintainer at [daniel@kerkmann.dev](mailto:daniel@kerkmann.dev).
