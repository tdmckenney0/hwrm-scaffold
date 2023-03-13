# Scaffold

![Kushan Mothership Scaffold](./scaffold.png)

Scaffold is a tool for developing mods for Homeworld: Remastered.

## Setup

You'll need a rust tool chain, which you can setup with help from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Once you have that, copy the `.env.example` file to `.env` and update `WEAPON_DIR` to point to **absolute path** to your Homeworld Remastered `Data/weapon` directory. Then run `cargo run`. Once it finishes, you should have a `*.sqlite` file that you can inspect with any SQL tool that supports sqlite. I recommend [dBeaver](https://dbeaver.io/) or [HeidiSQL](https://www.heidisql.com/).
