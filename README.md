# Scaffold

![Kushan Mothership Scaffold](./scaffold.png)

Scaffold is a tool for developing mods for Homeworld: Remastered.

## Setup

1. You'll need a rust tool chain, which you can setup with help from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). 
2. Copy the `.env.example` file to `.env`. 
3. Then run `cargo run -- --db test.sqlite --import-dir <PATH_TO_DATA_DIR>`.
4. Once it finishes, you should have a `test.sqlite` file that you can inspect with any SQL tool that supports sqlite. I recommend [dBeaver](https://dbeaver.io/) or [HeidiSQL](https://www.heidisql.com/).


