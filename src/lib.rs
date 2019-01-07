extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

mod universe;
use universe::Universe;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, world! wasm-glife!");
    let mut universe = universe::Universe::new(8, 8);
    log(&universe.sum().to_string());
    log(&universe.to_string());
    universe.tick();
    log(&universe.sum().to_string());
    log(&universe.to_string());
}

#[wasm_bindgen]
pub struct Game {
    universe: Universe,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            universe: Universe::new(width, height)
        }
    }

    #[allow(non_snake_case)]
    pub fn toString(&self) -> String {
        self.universe.to_string()
    }
    pub fn tick(&mut self) {
        self.universe.tick();
    }

    #[allow(non_snake_case)]
    pub fn isChanged(&self, i: usize) -> bool {
        self.universe.changed(i)
    }

    pub fn toggle(&mut self, i: usize) {
        self.universe.toggle(i);
    }
}
