pub mod grid;
pub mod infinite_grid;

use std::cmp::PartialEq;
use std::ops::Index;
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug)]
pub enum HorizontalVerticalDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Debug)]
pub enum HorizontalVerticalDiagonalDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

type HorizontalVerticalNeighbors<T> = Vec<((T, T), HorizontalVerticalDirection)>;
type HorizontalVerticalDiagonalNeighbors<T> = Vec<((T, T), HorizontalVerticalDiagonalDirection)>;

pub trait Neighbors {
    type Index: GridIndex;

    /// Gets the horizontal and vertical neighbors
    fn hv_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> HorizontalVerticalNeighbors<Self::Index>;

    /// Gets the horizontal, vertical, and diagonal neighbors
    fn hvd_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> HorizontalVerticalDiagonalNeighbors<Self::Index>;
}

pub trait GridIter {
    type GridRow;

    fn get_grid(&self) -> &Vec<Self::GridRow>;
    fn get_row_length(&self) -> usize;
    fn get_column_length(&self) -> usize;

    fn row_iter(&self) -> Iter<Self::GridRow> {
        self.get_grid().iter()
    }

    fn y_iter(&self) -> Iter<Self::GridRow> {
        self.get_grid().iter()
    }

    fn x_iter(&self) -> XIter<Self>
    where
        Self: Sized,
    {
        XIter::new(self)
    }

    fn column_iter(&self) -> XIter<Self>
    where
        Self: Sized,
    {
        XIter::new(self)
    }

    fn row_column_index_value_iter(&self) -> RowColumnIndexValueIter<Self>
    where
        Self: Sized,
    {
        RowColumnIndexValueIter::new(self)
    }

    fn x_y_value_iter(&self) -> XYValueIter<Self>
    where
        Self: Sized,
    {
        XYValueIter::new(self)
    }
}

pub trait GridIndex {}

impl GridIndex for usize {}
impl GridIndex for isize {}

#[must_use]
pub struct XIter<'g, G> {
    grid: &'g G,
    column_index: usize,
    column_length: usize,
}

impl<'g, G> Iterator for XIter<'g, G>
where
    G: GridIter,
    G::GridRow: Index<usize>,
{
    type Item = Vec<&'g <G::GridRow as Index<usize>>::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index < self.column_length {
            let column = self
                .grid
                .get_grid()
                .iter()
                .map(|row| &row[self.column_index])
                .collect();

            self.column_index += 1;

            Some(column)
        } else {
            None
        }
    }
}

impl<'g, G: GridIter> XIter<'g, G> {
    fn new(grid: &'g G) -> XIter<'g, G> {
        Self {
            grid,
            column_index: 0,
            column_length: grid.get_column_length(),
        }
    }
}

#[must_use]
pub struct XYValueIter<'g, G> {
    grid: &'g G,
    row_index: usize,
    row_length: usize,
    column_index: usize,
    column_length: usize,
}

impl<G: GridIter> XYValueIter<'_, G>
where
    G: GridIter + Index<usize, Output = G::GridRow>,
    G::GridRow: Index<usize>,
{
    pub fn find<P: Fn(&<G::GridRow as Index<usize>>::Output) -> bool>(
        &self,
        predicate: P,
    ) -> Option<(usize, usize)> {
        // The order here is incorrect, it goes by row by row, which is unexpected for xy (column by column)
        for ((row_index, column_index), v) in self.grid.row_column_index_value_iter() {
            if predicate(v) {
                return Some((row_index, column_index));
            }
        }

        None
    }
}

impl<'g, G: GridIter> XYValueIter<'g, G> {
    fn new(grid: &'g G) -> XYValueIter<'g, G> {
        Self {
            grid,
            row_index: 0,
            row_length: grid.get_row_length(),
            column_index: 0,
            column_length: grid.get_column_length(),
        }
    }
}

impl<'g, G> Iterator for XYValueIter<'g, G>
where
    G: GridIter + Index<usize, Output = G::GridRow>,
    G::GridRow: Index<usize>,
{
    type Item = ((usize, usize), &'g <G::GridRow as Index<usize>>::Output);

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index < self.column_length {
            let old = (
                (self.column_index, self.row_index),
                &self.grid[self.column_index][self.row_index],
            );

            // and go next
            if self.row_index + 1 == self.row_length {
                self.row_index = 0;

                self.column_index += 1;
            } else {
                self.row_index += 1;
            }

            Some(old)
        } else {
            None
        }
    }
}

#[must_use]
pub struct RowColumnIndexValueIter<'g, G> {
    grid: &'g G,
    row_index: usize,
    row_length: usize,
    column_index: usize,
    column_length: usize,
}

impl<G: GridIter> RowColumnIndexValueIter<'_, G>
where
    G: GridIter + Index<usize, Output = G::GridRow>,
    G::GridRow: Index<usize>,
{
    pub fn find<P: Fn(&<G::GridRow as Index<usize>>::Output) -> bool>(
        &self,
        predicate: P,
    ) -> Option<(usize, usize)> {
        for ((row_index, column_index), v) in self.grid.row_column_index_value_iter() {
            if predicate(v) {
                return Some((row_index, column_index));
            }
        }

        None
    }
}

impl<'g, G: GridIter> RowColumnIndexValueIter<'g, G> {
    fn new(grid: &'g G) -> RowColumnIndexValueIter<'g, G> {
        Self {
            grid,
            row_index: 0,
            row_length: grid.get_row_length(),
            column_index: 0,
            column_length: grid.get_column_length(),
        }
    }
}

impl<'g, G> Iterator for RowColumnIndexValueIter<'g, G>
where
    G: GridIter + Index<usize, Output = G::GridRow>,
    G::GridRow: Index<usize>,
{
    type Item = ((usize, usize), &'g <G::GridRow as Index<usize>>::Output);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.row_length {
            let old = (
                (self.row_index, self.column_index),
                &self.grid[self.row_index][self.column_index],
            );

            // and go next
            if self.column_index + 1 == self.column_length {
                self.column_index = 0;

                self.row_index += 1;
            } else {
                self.column_index += 1;
            }

            Some(old)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let first: Vec<char> = vec!['a', 'b', 'c'];

        let iter = first.iter();

        assert_eq!(vec![&'a', &'b', &'c'], iter.collect::<Vec<_>>());
    }
}
