# Digidog hosting server

This is just a small webserver to host the UniBas Programmierpojekt Game,
which is written in Java. Therefore, with this server you can download the
built jar file

## Prerequisites

To build this server, you need:

- [rust](https://www.rust-lang.org/tools/install) installed
- copy the `.env.example` to `.env` and change the respective env variables 

## Run it

`cargo run --release`

which will open up the server at `127.0.0.1:3000` by default.

## Deployment

### Prerequisites

You need:
- just: `cargo install just`
- [podman](https://podman.io/) 

Then setup letsencrypt with `just setup_letsencrypt`

## Steps—to be done initially

1. Setup letsencrypt with `just setup_letsencrypt`
2. Build the podman image with `just create_image`, so we can later build for aarch64-linux
3. Setup the systemd service with `just setup_systemd` 

You can also do all these steps with

```sh
just setup_all
```

## Steps—for further deployment

1. Deploy things with `just deploy_all`
2. Restart all services with `just start_all`
