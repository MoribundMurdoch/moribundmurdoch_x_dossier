#![allow(non_snake_case)]

mod analysis;
mod app;
mod config;
mod db;
mod export;
mod ingest;
mod public_data;
mod routes;
mod ui;

fn main() {
    dioxus::launch(app::App);
}
