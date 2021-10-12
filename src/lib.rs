extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod dictionary;
mod grid;
mod index;
mod solver;

use self::js_sys::Array;
use crate::dictionary::Dictionary;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct Solver {
    dict: Dictionary,
}

#[wasm_bindgen]
impl Solver {
    pub fn new(words_arr: Array) -> Solver {
        console_error_panic_hook::set_once();

        let words: Vec<String> = words_arr.iter().map(|d| d.as_string().unwrap()).collect();
        let dict = Dictionary::from_vec(words);

        Solver { dict }
    }

    pub fn solve(&self, spec_arr: Array) -> JsValue {
        let spec: Vec<Vec<usize>> = spec_arr
            .iter()
            .map(|a| {
                let ar: Array = a.into();

                ar.iter().map(|v| v.as_f64().unwrap() as usize).collect()
            })
            .collect();

        let grid = grid::Grid::new(spec);

        let result = solver::solve(&grid, &self.dict);

        if let Some(r) = result {
            let v: Vec<JsValue> = r.iter().map(|c| c.to_string().into()).collect();
            let a: Array = v.iter().collect();

            a.into()
        } else {
            JsValue::NULL
        }
    }
}
