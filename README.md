# Media Share

A service for sharing files locally through a web-based interface. Upload a file to the server, then retrieve it from a unique endpoint. It also supports expiry--deleting files after a predetermined amount of time. This is ideal for cases where you need to share a file between different devices quickly.

## Deployment

This service can be deployed using docker compose. Simply build the image in the root directory of the project using `docker compose build`, then run using `docker compose up`, and you should be able to access the service at the default port, which is 8080.

You may set the amount of hours you would like a file to last before expiration as `EXPIRY_HOURS=` in `.env`. To actually implement this functionaltiy, you will need to set up a scheduled (likely a cron) job that runs the `src/bin/delete_expired.rs` script. This script will delete any expired records and remove the corresponding files.

Visit `/upload` to upload a file, and use `/get/:label` to get a file by its label--a file label is a randomly generated pair of words used to unique identify a file.

## Technologies Used

All of the business logic is implemented in pure Rust:

### Frontend

The frontend pages are written using plain HTML/CSS templates. The backend renders and inserts data into these templates using the [askama](https://crates.io/crates/askama) crate.

### Backend

On the backend, this uses the [axum](https://crates.io/crates/axum) web framework with the [tokio](https://tokio.rs/) async runtime.

### Database

The backend uses the [sqlx](https://crates.io/crates/sqlx) crate to interface with a Postgres database. This database holds records of all the files that have been uploaded and their locations on the server. Uploaded files are simply stored on the filesystem, in the directory specified by `FILES_DIR`
