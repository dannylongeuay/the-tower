use crate::position::Position;

pub struct Grid<T> {
    width: usize,
    height: usize,
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
        let index = self.index(pos);
        self.cells.get(index)
    }

    pub fn get_mut(&mut self, pos: &Position) -> Option<&mut T> {
        let index = self.index(pos);
        self.cells.get_mut(index)
    }

    fn index(&self, pos: &Position) -> usize {
        pos.y as usize * self.width + pos.x as usize
    }
}
