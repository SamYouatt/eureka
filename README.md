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

## Deployment

### Postgres

I am using fly postgres to run a sem-managed db instance.

To check if the db is currently running on fly run `fly postgres list`.

> The connection information for the running database is stored in Bitwarden. Don't publish anywhere!

In order to connect to the running instance locally:
- Run `fly proxy 54321:5432 -a sam-y-eureka-pg` in order to open up a wireguard tunnel through on `localhost:54321` (this needs to be kept opening for the tunnel to exist)
- In a db client like DataGrip, set up as following:
- Host: localhost
- Username: postgres
- Password: check bitwarden
- Port: 54321
- Version: 14.0 should work but I'm not actually sure how to tell for certain what version fly is running
