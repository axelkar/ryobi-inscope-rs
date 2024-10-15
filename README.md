# ryobi-inscope-rs

![Screenshot](screenshot.png)

Command-line program for working with RYOBI's Phone Works inspection scope.

More Phone Works tools could be added later.

## Usage

Requires ffplay from ffmpeg and currently only works on Unix-like platforms.

1. Turn on the inspection scope by pressing on the button for 5 seconds
2. Connect to its Wi-Fi network (default password is 12345678)
3. Run this program

Keybinds:

* <kbd>q</kbd> Quit the program
* <kbd>0</kbd> Turn off the light
* <kbd>1</kbd> Set the light level to 1
* <kbd>2</kbd> Set the light level to 2
* <kbd>3</kbd> Set the light level to 3
* <kbd>↑</kbd> Increase the light level
* <kbd>↓</kbd> Decrease the light level

## Development

0. Have Linux or MacOS

1. Install [Nix](https://nixos.org/download#download-nix)

2. Run the command `nix develop` in a shell.

   This creates a `bash` subshell with all the dependencies.

3. Run `cargo` commands as you like.

   i.e. `cargo build`, `cargo run`, `cargo clippy`, etc.

## Contributing patches

Please first make sure that you have not introduced any regressions and format the code by running the following commands at the repository root.
```sh
cargo fmt
cargo clippy
cargo test
```

You can either make a GitHub [pull request](https://github.com/axelkar/ryobi-inscope-rs/pulls) or email me directly:

0. Setup `git send-email`:

   <https://git-send-email.io/>

1. Commit your changes, this will open up a text editor

   `git commit`

2. Send your patches to me. The command sends the last commit

   `git send-email --to="axel@axka.fi" HEAD^`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
