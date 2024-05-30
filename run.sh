#!/bin/bash
sqlx database create
sqlx migrate run
./target/release/media_share