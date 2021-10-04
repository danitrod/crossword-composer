use crate::dictionary::Dictionary;
use crate::grid::Grid;
use crate::index::Index;
use std::collections::BTreeSet;

struct SolverStep {
    pub index: Index,
    pub input_slots: Vec<usize>, // Vector of slot indices to pull as input.
    pub output_slots: Vec<usize>, // Vector of slot indices to put output in.
}

struct SolverState {
    result: Vec<char>,
    words: Vec<usize>,
}

impl SolverState {
    pub fn new(grid: &Grid) -> SolverState {
        let result = vec![' '; grid.slots];
        let words = vec![0; grid.num_words()];

        SolverState { result, words }
    }
}

fn generate_solver_steps(grid: &Grid, dict: &Dictionary) -> Vec<SolverStep> {
    let mut solver_steps: Vec<SolverStep> = Vec::with_capacity(grid.num_words());

    // Word index -> number of constraints on this word.
    let mut constraints = vec![0; grid.slots];

    // Set of word indices that do not yet have steps associated with them.
    let mut remaining_words: BTreeSet<usize> = (0..grid.num_words()).collect();

    // Slots which are considered decided by the current point in the solving order.
    let mut known_slots: BTreeSet<usize> = BTreeSet::new();

    while remaining_words.len() > 0 {
        let max_idx: usize = *remaining_words
            .iter()
            .max_by_key(|i| {
                (constraints[**i], grid.words[**i].len()) // Find max first by number of constraints, then length.
            })
            .unwrap();

        remaining_words.remove(&max_idx);

        // We know what is the next word to process (the one with most constraints).
        // Now try to find it and go to the next one.
        let mut known_letters = Vec::new();
        let mut input_slots: Vec<usize> = Vec::new();
        let mut output_slots: Vec<usize> = Vec::new();
        for (i, slot_id) in grid.words[max_idx].iter().enumerate() {
            if known_slots.contains(slot_id) {
                // If we already know what letter should be in this slot, use the letter.
                known_letters.push(i);
                input_slots.push(*slot_id);
            } else {
                // If not, it is a new slot
                known_slots.insert(*slot_id);
                output_slots.push(*slot_id);
            }

            for word_id in &grid.slot_to_words[*slot_id] {
                // Add the new constraints for future words
                constraints[*word_id] += 1;
            }
        }

        // index is a helper struct to better search through the dictionary.
        let index = Index::new(known_letters, grid.words[max_idx].len(), &dict);
        solver_steps.push(SolverStep {
            index,
            input_slots,
            output_slots,
        })
    }

    solver_steps
}

fn solve_step(state: &mut SolverState, steps: &Vec<SolverStep>, step: usize) -> bool {
    if step >= steps.len() {
        true
    } else {
        let SolverStep {
            index,
            input_slots,
            output_slots,
        } = &steps[step];

        let known_letters: Vec<char> = input_slots.iter().map(|j| state.result[*j]).collect();

        for (wi, attempt) in index.get(&known_letters) {
            if state.words[0..step].contains(wi) {
                // Words can't be reused
                continue;
            }
            state.words[step] = *wi;

            for (out_slot, att) in output_slots.iter().zip(attempt) {
                state.result[*out_slot] = *att;
            }

            if solve_step(state, steps, step + 1) {
                return true;
            }
        }

        false
    }
}

pub fn solve(grid: &Grid, dict: &Dictionary) -> Option<Vec<char>> {
    // Takes: grid (list of words with slots) + dict (a map with key being an int and value being all words with key as length)
    let mut state: SolverState = SolverState::new(&grid);
    let steps = generate_solver_steps(&grid, &dict);

    if solve_step(&mut state, &steps, 0) {
        Some(state.result)
    } else {
        None
    }
}
