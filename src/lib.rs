extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod dictionary;
mod grid;
mod index;
mod par_solver;
mod solver;

use self::js_sys::Array;
use crate::dictionary::Dictionary;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_simple_crossword() {
        let slot_words = vec![
            vec![0, 1, 2, 3],
            vec![4, 0, 5, 6],
            vec![7, 8, 5, 9],
            vec![10, 11, 1, 12, 5],
        ];
        let grid = grid::Grid::new(slot_words.clone());

        let words: Vec<String> = std::fs::read_to_string("./ui/public/words.txt")
            .unwrap()
            .split('\n')
            .map(|str| str.to_string())
            .collect();

        let result = solver::solve(&grid, &Dictionary::from_vec(words)).unwrap();

        println!(
            "Result: {:?}",
            slot_words
                .into_iter()
                .map(|v| v.into_iter().map(|n| result[n]).collect())
                .collect::<Vec<Vec<char>>>()
        );

        // This test assumes a deterministic approach in which everytime the same words will be chosen
        assert_eq!(
            vec!['a', 'g', 'e', 's', 'd', 't', 'a', 'w', 'i', 'h', 'e', 'i', 'h'],
            result
        );
    }

    #[test]
    fn sequential_complex_crossword() {
        let slot_words = vec![
            vec![0, 1, 2, 3],
            vec![4, 0, 5, 6],
            vec![7, 8, 5, 9],
            vec![10, 11, 1, 12, 5],
            vec![13, 8, 14, 12, 15],
            vec![16, 9, 2, 17, 18],
            vec![16, 1, 0, 8, 11, 14],
            vec![19, 5, 20, 21, 13, 22],
            // vec![12, 3, 14, 11, 15, 20, 23],
            // vec![5, 4, 9],
            // vec![24, 25, 12],
        ];
        let grid = grid::Grid::new(slot_words.clone());

        let words: Vec<String> = std::fs::read_to_string("./ui/public/words.txt")
            .unwrap()
            .split('\n')
            .map(|str| str.to_string())
            .collect();

        let result = solver::solve(&grid, &Dictionary::from_vec(words)).unwrap();

        println!(
            "Result: {:?}",
            slot_words
                .into_iter()
                .map(|v| v.into_iter().map(|n| result[n]).collect())
                .collect::<Vec<Vec<char>>>()
        );

        // This test assumes a deterministic approach in which everytime the same words will be chosen
        assert_eq!(
            vec![
                'o', 'r', 'a', 'l', 'b', 't', 'h', 'h', 'd', 'v', 'm', 'e', 'i', 'e', 'd', 'e',
                'e', 'n', 's', 's', 'a', 't', 's'
            ],
            result
        );
    }
}
