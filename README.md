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

### Tailwind

To luanch the tailwind compiler in watch mode run `npx tailwindcss -i ./styles/tailwind.css -o ./assets/main.css --watch`. This will rebuild the `app/main.css` based on whatever has changed.

### Supabase

Chose to use Supabase as it looked like a nice way of hosting a Postgres instance. To launch the db locally run: `supabase start` with the docker daemon running.
