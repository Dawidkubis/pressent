// TODO LEARN SEED
use seed::{*, prelude::*};

// Model
struct Model;

dhg
// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self
    }
}


// Update
#[derive(Clone)]
enum Msg {
    KeyDown(web_sys::KeyboardEvent)
}

/// How we update the model
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
    	KeyDown(key) => //hmm
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}

