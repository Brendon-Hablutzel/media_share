# Media Share

A service for sharing files locally through a web-based interface. Upload a file to the server, then retrieve it from a unique endpoint. It also supports expiry--deleting files after a predetermined amount of time. This is ideal for cases where you need to share a file between different devices quickly.

## Deployment

This service is intended to be self-hosted locally and should not be used to store any sensitive data, as it does not (yet) contain any functionality for auth.

Before running this service, you must first set up postgres and ensure it is running correctly. Then you will need to create a user with appropriate permissions, and put all the corresponding database uri in the `.env` file as `DATABASE_URL=...`. See `.env.template` for an example.

To create the database and appropriate table, install the [sqlx cli](https://crates.io/crates/sqlx-cli/0.5.2), then run `sqlx database create` to create the database, and `sqlx migrate run` to run the migrations, which should create the table. Note that the sqlx queries in the code will not compile until the database is set up, since they check for query validity at compile time.

You will also need to provide a file directory path, which is where the files will be stored. Assign this value to `FILES_DIR=` in `.env`.

Then, set the amount of hours you would like a file to last before expiration as `EXPIRY_HOURS=` in `.env`. To actually implement this functionaltiy, you will need to set up a scheduled (likely a cron) job that runs the `src/bin/delete_expired.rs` script. This script will delete any expired records and remove the corresponding files.

In `.env`, set `PORT=` to the port you would like to run the web server on.

Finally, to start the service, simply run `main.rs` and the web server will begin listening for connections. Visit `/upload` to upload a file, and use `/get/:label` to get a file by its label--a file label is a randomly generated pair of words used to unique identify a file.

## Technologies Used

All of the business logic is implemented in pure Rust:

### Frontend

The frontend pages are written using plain HTML/CSS templates. The backend renders and inserts data into these templates using the [askama](https://crates.io/crates/askama) crate.

### Backend

On the backend, this uses the [axum](https://crates.io/crates/axum) web framework with the [tokio](https://tokio.rs/) async runtime.

### Database

The backend uses the [sqlx](https://crates.io/crates/sqlx) crate to interface with a Postgres database. This database holds records of all the files that have been uploaded and their locations on the server. Uploaded files are simply stored on the filesystem, in the directory specified by `FILES_DIR`
