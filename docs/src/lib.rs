#![recursion_limit = "4096"]
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod routes;
mod layouts;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
