use crossword::{dictionary::Dictionary, grid, solver};

fn main() {
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
        vec![1, 20, 30, 40, 38, 45],
        vec![35, 41],
        vec![40, 41, 42, 43, 44, 45],
        vec![39, 41, 42, 21, 22, 45],
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
