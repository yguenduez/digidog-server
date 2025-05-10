# Digidog hosting server

This is just a small webserver to host the UniBas Programmierpojekt Game,
which is written in Java. Therefore, with this server you can download the
built jar file

## Prerequisites

To build this server you need:

- [rust](https://www.rust-lang.org/tools/install) installed
- copy the `.env.example` to `.env` and change the respective filepath of the directory with the jar file in it

## Run it

`cargo run --release`

which will open up the server at `0.0.0.0:3000` by default.