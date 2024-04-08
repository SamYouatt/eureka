# Eureka

Eureka is a project aimed mostly at exploring some technologies that have peaked my interest and I want to get something substantial built with.

However the purpose of the app itself is to track your ideas and aim to introduce a simple framework that might help to encourage continued progress.

## Tech stack

Introducing the PHART stack:
- `PostgresDb`
- `HTMX`
- `Axum`
- `Rust`
- `Tailwind`

## Running locally

### App

To run the main app simply run `cargo run`. For development I have been using `cargo watch -x check -x test -x run` for quick feedback. Any of `-x step` can be excluded if not required.

> Actually I have now started using `cargo watch -x check -x test -x run | bunyan` for its nicer formatting of the tracing

### Tailwind

To launch the tailwind compiler in watch mode run `npx tailwindcss -i ./styles/tailwind.css -o ./assets/main.css --watch`. This will rebuild the `app/main.css` based on whatever has changed.

### Postgres

To spin up a local postgres instance for the db and run all the migrations, run the `init_db.sh` script. It has an optional flag to skip docker if an instance of the db is already running - useful if you want to run migrations again without shutting down the db.

`./scripts/init_db.sh` or `SKIP_DOCKER=true ./scripts/init_db.sh`
