Typeous
=======

Typeous is a minimalistic command line typing tool written in Rust.

Installation
------------

1. Make sure you have Rust and Cargo installed
2. `git clone` the repository to your computer
3. `cd` into the repository
4. Run `cargo build`
5. To run typeous run `cargo run`

Usage
-----

1. Find the text you want to practice on and copy it to `typefile.txt`.
2. Run typeous with `cargo run`
3. Touch type the text
4. See the statistics

If you want to type the text from a file, run
`cargo run -- the/file/path/to/your/file`

Configuration
-------------

You can edit the statistics report to show only the information you want.
To do that go to `config.yaml` and edit the `statistics` list.

The available options are:

- `chars`, the number of characters
- `words`, the number of words
- `lines`, the number of lines
- `millis`, time in milliseconds
- `seconds`, time in seconds
- `cpm`, speed in characters per minute
- `wpm`, speed in words per minute
