use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use rand::{seq::SliceRandom, Rng};

use crate::{
    position::{Position, DIRECTIONS},
    tower::grid::Grid,
};

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
        other.entropy.partial_cmp(&self.entropy)
    }
}

#[derive(Debug)]
pub struct Cell {
    possible: Vec<bool>,
    total_weight: f32,
    total_weight_log_weight: f32,
    entropy_noise: f32,
    pub chosen_index: Option<usize>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            possible: Vec::new(),
            total_weight: 0.,
            total_weight_log_weight: 0.,
            entropy_noise: 0.,
            chosen_index: None,
        }
    }
}

impl Cell {
    fn total_weight(&self, d: &Vec<usize>) -> f32 {
        let mut total = 0.;
        for (index, &possible) in self.possible.iter().enumerate() {
            if possible {
                total += *d.get(index).unwrap() as f32;
            }
        }
        return total;
    }
    fn total_weight_log_weight(&self, d: &Vec<usize>) -> f32 {
        self.possible
            .iter()
            .enumerate()
            .map(|(index, &possible)| {
                if possible {
                    let rf = *d.get(index).unwrap() as f32;
                    return rf * rf.log2();
                } else {
                    return 0.;
                }
            })
            .sum()
    }
    fn set_entropy(&mut self, d: &Vec<usize>, noise: f32) {
        self.total_weight = self.total_weight(d);
        self.total_weight_log_weight = self.total_weight_log_weight(d);
        self.entropy_noise = noise;
    }
    fn entropy(&self) -> f32 {
        self.total_weight.log2() - self.total_weight_log_weight / self.total_weight
            + self.entropy_noise
    }
    fn remove_index(&mut self, index: usize, d: &Vec<usize>) {
        self.possible[index] = false;
        let rf = *d.get(index).unwrap() as f32;
        self.total_weight -= rf;
        self.total_weight_log_weight -= rf * rf.log2();
    }
    fn collapse(&mut self, chosen_index: usize, d: &Vec<usize>) {
        self.chosen_index = Some(chosen_index);
        let index_removals: Vec<usize> = self
            .possible
            .iter()
            .enumerate()
            .filter(|(i, &s)| *i != chosen_index && s)
            .map(|(i, _)| i)
            .collect();
        for index in index_removals {
            self.remove_index(index, d);
        }
    }
    fn choose_possible(&self, d: &Vec<usize>) -> Option<usize> {
        let mut choices: Vec<usize> = Vec::new();
        for (index, &status) in self.possible.iter().enumerate() {
            if status {
                let dist = *d.get(index).unwrap();
                for _ in 0..dist {
                    choices.push(index);
                }
            }
        }
        choices.choose(&mut rand::thread_rng()).copied()
    }
}

pub struct WFC {
    pub cells: Grid<Cell>,
    pub uncollapsed_cells: usize,
    constraints: Vec<Vec<(Position, Vec<usize>)>>,
    distributions: Vec<usize>,
    entropy_heap: BinaryHeap<EntropyPosition>,
}

impl WFC {
    pub fn new(
        width: usize,
        height: usize,
        constraints: Vec<Vec<(Position, Vec<usize>)>>,
        distributions: Vec<usize>,
    ) -> Self {
        let mut entropy_heap: BinaryHeap<EntropyPosition> = BinaryHeap::new();
        let mut cells: Grid<Cell> = Grid::new(width, height);
        let mut rng = rand::thread_rng();
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x as i32, y as i32);
                let cell = cells.get_mut(&pos).unwrap();
                cell.possible.resize(distributions.len(), true);
                cell.set_entropy(&distributions, rng.gen_range(0.001..0.01));
                entropy_heap.push(EntropyPosition {
                    entropy: cell.entropy(),
                    pos,
                })
            }
        }
        let uncollapsed_cells = cells.len();
        WFC {
            cells,
            uncollapsed_cells,
            constraints,
            distributions,
            entropy_heap,
        }
    }
    fn choose(&mut self) -> Position {
        while let Some(EntropyPosition { entropy: _, pos }) = self.entropy_heap.pop() {
            let cell = self.cells.get(&pos).unwrap();
            if cell.chosen_index.is_none() {
                return pos;
            }
        }
        unreachable!("entropy heap is empty");
    }
    fn collapse(&mut self, pos: &Position) {
        let cell = self.cells.get_mut(pos).unwrap();
        if let Some(chosen_index) = cell.choose_possible(&self.distributions) {
            cell.collapse(chosen_index, &self.distributions);
        } else {
            panic!("cannot collapse cell at {:?}", pos);
        }
    }
    fn propagate(&mut self, pos: Position) {
        let mut stack = vec![pos];
        while let Some(pos) = stack.pop() {
            for dir in DIRECTIONS {
                let current_cel = self.cells.get(&pos).unwrap();
                let neighbor = pos + *dir;
                if !self.cells.in_bounds(&neighbor) {
                    continue;
                }
                let mut possible_neighbors: Vec<usize> = Vec::new();
                for (i, p) in current_cel.possible.iter().enumerate() {
                    if !p {
                        continue;
                    }
                    let constraint = self.constraints.get(i).unwrap();
                    for (c_dir, allowed_sockets) in constraint {
                        if *c_dir != *dir {
                            continue;
                        }
                        possible_neighbors.extend(allowed_sockets);
                    }
                }
                let neighbor_cell = self.cells.get_mut(&neighbor).unwrap();
                if neighbor_cell.chosen_index.is_some() {
                    continue;
                }
                let indexes: Vec<usize> = neighbor_cell
                    .possible
                    .iter()
                    .enumerate()
                    .filter(|(_, &s)| s)
                    .map(|(i, _)| i)
                    .collect();

                for index in indexes {
                    if !possible_neighbors.contains(&index) {
                        neighbor_cell.remove_index(index, &self.distributions);
                        self.entropy_heap.push(EntropyPosition {
                            entropy: neighbor_cell.entropy(),
                            pos: neighbor,
                        });
                        if !stack.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for WFC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.cells.height {
            for x in 0..self.cells.width {
                let cell = self.cells.get(&Position::new(x as i32, y as i32)).unwrap();
                let possible: Vec<usize> = cell
                    .possible
                    .iter()
                    .enumerate()
                    .filter(|(_i, &p)| p)
                    .map(|(i, _p)| i)
                    .collect();
                let s = String::from_iter(possible.iter().map(|&i| i.to_string()));
                write!(f, "{s:03} ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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
        self.propagate(next_pos);
        self.uncollapsed_cells -= 1;
        Some(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_basic_wfc_tiled() {
        let constraints: Vec<Vec<(Position, Vec<usize>)>> = vec![
            vec![
                (Position::UP, vec![0, 1]),
                (Position::DOWN, vec![0, 1]),
                (Position::LEFT, vec![0, 1]),
                (Position::RIGHT, vec![0, 1]),
            ],
            vec![
                (Position::UP, vec![0, 1, 2]),
                (Position::DOWN, vec![0, 1, 2]),
                (Position::LEFT, vec![0, 1, 2]),
                (Position::RIGHT, vec![0, 1, 2]),
            ],
            vec![
                (Position::UP, vec![1, 2]),
                (Position::DOWN, vec![1, 2]),
                (Position::LEFT, vec![1, 2]),
                (Position::RIGHT, vec![1, 2]),
            ],
        ];
        let mut wfc = WFC::new(5, 5, constraints, vec![1, 1, 1]);
        while wfc.next().is_some() {
            println!("{}", wfc);
        }
        assert!(wfc.next().is_none());
    }
}
