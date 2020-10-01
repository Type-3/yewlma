#![recursion_limit = "4096"]
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod layouts;
mod pages;
mod routes;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
