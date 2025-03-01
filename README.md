Typeous
=======

Typeous is a minimalistic command line typing tool written in Rust.

[![asciicast](https://asciinema.org/a/G7ZevaiXFRU8nwouQRnAfsTfR.png)](https://asciinema.org/a/G7ZevaiXFRU8nwouQRnAfsTfR)

Installation
------------

1. Make sure you have Rust and Cargo installed
2. `git clone` the repository to your computer
3. `cd` into the repository
4. Run `cargo build`
5. To run typeous run `cargo run`

Usage
-----

1. Create a file called `typeous.txt` in your home directory
1. Find the text you want to practice on and copy it to `typeous.txt`.
2. Run typeous with `cargo run`
3. Touch type the text
4. See the statistics

Configuration
-------------

You can edit the statistics report to show the information you want.
To do that go to `typeous/config.yaml` in your config directory
(`.config` for Linux) and edit the `statistics` list.

The available options are:

- `chars`, the number of characters
- `words`, the number of words
- `lines`, the number of lines
- `millis`, time in milliseconds
- `seconds`, time in seconds
- `cpm`, speed in characters per minute
- `wpm`, speed in words per minute

If you are not sure where is the `config.yaml` file, run `cargo run -- --help`.
