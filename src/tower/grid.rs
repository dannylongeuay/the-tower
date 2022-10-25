use crate::position::Position;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let mut cells: Vec<T> = Vec::new();
        cells.resize_with(width * height, T::default);
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<&T> {
        if !self.in_bounds(pos) {
            return None;
        }
        let index = self.index(pos);
        self.cells.get(index)
    }

    pub fn get_mut(&mut self, pos: &Position) -> Option<&mut T> {
        if !self.in_bounds(pos) {
            return None;
        }
        let index = self.index(pos);
        self.cells.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut()
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    fn index(&self, pos: &Position) -> usize {
        pos.y as usize * self.width + pos.x as usize
    }
}
