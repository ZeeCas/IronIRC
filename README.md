# IronIRC
IRC Client written in Rust using [ratatui](https://crates.io/crates/ratatui) and [IRC](https://crates.io/crates/irc).

## Usage :
* Clone Repo
* Create config.toml (see IRC crates.io) in root of the repo
```toml
nickname = "Nickname-Here"
password = "password" # Password is not needed unless connecting to a server that supports sasl plain authentication
server = "Enter Server URL"
channels = ['#Channel1', '#Channel2', '#Channel3']

[options]
on_join = "NickServ IDENTIFY username password" # This is an example of an onjoin command, if this is specified in your config
                                                # the client will send a message to the 'recipient' containing the body, where
                                                # 'recipient' is the first word, and the body is the rest of the definition.
```
* Execute ```cargo run```
* ???
* Profit