use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use rand::seq::SliceRandom;

use crate::position::Position;

struct Contraints;

struct Frequencies;

impl Frequencies {
    fn relative_frequency(&self, index: usize) -> f32 {
        1.
    }
}

#[derive(PartialEq)]
struct EntropyPosition {
    entropy: f32,
    pos: Position,
}

impl Eq for EntropyPosition {}

impl Ord for EntropyPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entropy.total_cmp(&other.entropy)
    }
}

impl PartialOrd for EntropyPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Cell {
    pos: Position,
    possible: Vec<bool>,
    total_weight: f32,
    total_weight_log_weight: f32,
    entropy_noise: f32,
    collapsed: bool,
}

impl Cell {
    fn total_frequencies(&self, f: &Frequencies) -> f32 {
        let mut total = 0.;
        for (index, &possible) in self.possible.iter().enumerate() {
            if possible {
                total += f.relative_frequency(index);
            }
        }
        return total;
    }
    fn entropy(&self, f: &Frequencies) -> f32 {
        // move this to initialization
        {
            let total_weight = self.total_frequencies(f) as f32;
            let total_weight_log_weight: f32 = self
                .possible
                .iter()
                .enumerate()
                .map(|(index, &possible)| {
                    if possible {
                        let rf = f.relative_frequency(index) as f32;
                        return rf * rf.log2();
                    } else {
                        return 0.;
                    }
                })
                .sum();
            return total_weight.log2() - (total_weight_log_weight / total_weight);
        }
        // self.total_weight.log2() - (self.log_weight / self.total_weight)
    }
    fn remove_tile(&mut self, index: usize, f: &Frequencies) {
        self.possible[index] = false;
        let rf = f.relative_frequency(index);
        self.total_weight -= rf;
        self.total_weight_log_weight -= rf * rf.log2();
    }
    fn choose_possible(&self, f: &Frequencies) -> Option<usize> {
        let mut choices: Vec<usize> = Vec::new();
        for (index, &status) in self.possible.iter().enumerate() {
            if status {
                choices.push(index);
            }
        }
        choices.choose(&mut rand::thread_rng()).copied()
    }
}

struct WFC {
    cells: HashMap<Position, Cell>,
    uncollapsed_cells: usize,
    contraints: Contraints,
    frequencies: Frequencies,
    entropy_heap: BinaryHeap<EntropyPosition>,
}

impl WFC {
    fn choose(&mut self) -> Position {
        while let Some(EntropyPosition { entropy: _, pos }) = self.entropy_heap.pop() {
            let cell = self.cells.get(&pos).unwrap();
            if !cell.collapsed {
                return pos;
            }
        }
        unreachable!("entropy heap is empty");
    }
    fn collapse(&mut self, pos: &Position) {
        let mut cell = self.cells.get_mut(pos).unwrap();
        if let Some(chosen_index) = cell.choose_possible(&self.frequencies) {
            cell.collapsed = true;
            for (index, status) in cell.possible.iter_mut().enumerate() {
                if chosen_index == index {
                    continue;
                }
                *status = false;
            }
        } else {
            panic!("cannot collapse cell at {:?}", pos);
        }
    }
    fn propagate(&mut self) {
        todo!()
    }
}

impl Iterator for WFC {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        if self.uncollapsed_cells == 0 {
            return None;
        }
        let next_pos = self.choose();
        self.collapse(&next_pos);
        self.propagate();
        self.uncollapsed_cells -= 1;
        Some(())
    }
}
