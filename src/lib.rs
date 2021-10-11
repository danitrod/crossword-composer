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

        let result = solver::solve(&grid, &Dictionary::from_vec(words));

        assert!(result.is_some());

        let result = result.unwrap();

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

        let result = solver::solve(&grid, &Dictionary::from_vec(words));
        assert!(result.is_some());
        let result = result.unwrap();

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

    #[test]
    fn sequential_super_complex_crossword() {
        let slot_words = vec![
            vec![0, 6, 12, 16, 23],
            vec![0, 1, 2, 3, 4, 5],
            vec![1, 7, 13, 17, 24],
            vec![2, 8, 14, 18, 25],
            vec![3, 9, 15, 19],
            vec![4, 10],
            vec![5, 11],
            vec![6, 7, 8, 9, 10, 11],
            vec![12, 13, 14, 15],
            vec![16, 17, 18, 19],
            vec![20, 27, 31, 37, 43],
            vec![20, 21, 22],
            vec![21, 28, 32, 38, 44],
            vec![22, 29, 33, 39, 45],
            vec![23, 24, 25],
            vec![26, 30, 36, 42],
            vec![26, 27, 28, 29],
            vec![30, 31, 32, 33],
            vec![34, 40],
            vec![34, 35, 36, 37, 38, 39],
            vec![35, 41],
            vec![40, 41, 42, 43, 44, 45],
        ];
        let grid = grid::Grid::new(slot_words.clone());

        let words: Vec<String> = std::fs::read_to_string("./ui/public/words.txt")
            .unwrap()
            .split('\n')
            .map(|str| str.to_string())
            .collect();

        let result = solver::solve(&grid, &Dictionary::from_vec(words));
        assert!(result.is_some());
        let result = result.unwrap();

        println!(
            "Result: {:?}",
            slot_words
                .into_iter()
                .map(|v| v.into_iter().map(|n| result[n]).collect())
                .collect::<Vec<Vec<char>>>()
        );
    }
}
