mod app;
mod generated;
mod samples;

use seed::{prelude::*, *};

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::build(app::init, app::update, app::view).finish().run();

    log!("App started.");
}
