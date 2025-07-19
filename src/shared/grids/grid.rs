use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::{
    GridIter, HorizontalVerticalDiagonalDirection, HorizontalVerticalDiagonalNeighbors,
    HorizontalVerticalDirection, HorizontalVerticalNeighbors, Neighbors,
};

pub struct Grid<T> {
    data: Vec<Row<T>>,
    row_len: usize,
    column_len: usize,
    // max_row: usize,
    // max_column: usize,
}

impl<T> Deref for Grid<T> {
    type Target = [Row<T>];

    fn deref(&self) -> &[Row<T>] {
        self.data.as_slice()
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut [Row<T>] {
        self.data.as_mut_slice()
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            row_len: self.row_len,
            column_len: self.column_len,
        }
    }
}

impl<T> Grid<T> {
    /// Builds a new grid
    ///
    /// # Panics
    /// When rows are not equal length
    #[must_use]
    pub fn new(data: Vec<Vec<T>>) -> Self {
        for w in data.windows(2) {
            assert_eq!(w[0].len(), w[1].len());
        }

        let rows = data.len();
        let columns = data[0].len();

        Self {
            data: data.into_iter().map(|r| Row(r)).collect(),
            row_len: rows,
            column_len: columns,
            // max_row: rows - 1,
            // max_column: columns - 1,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Row<T>(Vec<T>);

impl<T: Clone> std::clone::Clone for Row<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Row<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Deref for Row<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> DerefMut for Row<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }
}

impl<T> GridIter for Grid<T> {
    type GridRow = Row<T>;

    fn get_grid(&self) -> &Vec<Self::GridRow> {
        &self.data
    }

    fn get_row_length(&self) -> usize {
        self.row_len
    }

    fn get_column_length(&self) -> usize {
        self.column_len
    }
}

impl<T> Neighbors for Grid<T> {
    type Index = usize;

    fn hv_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> HorizontalVerticalNeighbors<Self::Index> {
        let mut neighbors = vec![];

        let up = row_index.checked_sub(1);
        let down = {
            let down = row_index + 1;

            (down < self.row_len).then_some(down)
        };

        let left = column_index.checked_sub(1);
        let right = {
            let right = column_index + 1;

            (right < self.column_len).then_some(right)
        };

        if let Some(up) = up {
            neighbors.push(((up, column_index), HorizontalVerticalDirection::Up));
        }

        if let Some(right) = right {
            neighbors.push(((row_index, right), HorizontalVerticalDirection::Right));
        }

        if let Some(down) = down {
            neighbors.push(((down, column_index), HorizontalVerticalDirection::Down));
        }

        if let Some(left) = left {
            neighbors.push(((row_index, left), HorizontalVerticalDirection::Left));
        }

        neighbors
    }

    fn hvd_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> HorizontalVerticalDiagonalNeighbors<Self::Index> {
        let mut neighbors = vec![];

        let up = row_index.checked_sub(1);
        let down = {
            let down = row_index + 1;

            (down < self.row_len).then_some(down)
        };

        let left = column_index.checked_sub(1);
        let right = {
            let right = column_index + 1;

            (right < self.column_len).then_some(right)
        };

        if let Some(up) = up {
            neighbors.push(((up, column_index), HorizontalVerticalDiagonalDirection::Up));
        }

        if let (Some(up), Some(right)) = (up, right) {
            neighbors.push(((up, right), HorizontalVerticalDiagonalDirection::UpRight));
        }

        if let Some(right) = right {
            neighbors.push((
                (row_index, right),
                HorizontalVerticalDiagonalDirection::Right,
            ));
        }

        if let (Some(down), Some(right)) = (down, right) {
            neighbors.push((
                (down, right),
                HorizontalVerticalDiagonalDirection::DownRight,
            ));
        }

        if let Some(down) = down {
            neighbors.push((
                (down, column_index),
                HorizontalVerticalDiagonalDirection::Down,
            ));
        }

        if let (Some(down), Some(left)) = (down, left) {
            neighbors.push(((down, left), HorizontalVerticalDiagonalDirection::DownLeft));
        }

        if let Some(left) = left {
            neighbors.push(((row_index, left), HorizontalVerticalDiagonalDirection::Left));
        }

        if let (Some(up), Some(left)) = (up, left) {
            neighbors.push(((up, left), HorizontalVerticalDiagonalDirection::UpLeft));
        }

        neighbors
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for t in &row.0 {
                write!(f, "{}", t)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rows: {}, Columns: {}", self.row_len, self.column_len)?;
        for row in &self.data {
            for t in &row.0 {
                write!(f, "{:?}", t)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Row<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::shared::grids::{
        GridIter as _, HorizontalVerticalDiagonalDirection, HorizontalVerticalDirection,
        Neighbors as _,
    };

    #[test]
    fn rows() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let rows = g.data.iter().collect::<Vec<_>>();

        assert_eq!(rows, g.row_iter().collect::<Vec<_>>());
    }

    #[test]
    fn columns() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let columns = vec![
            vec![&'a', &'d', &'g'],
            vec![&'b', &'e', &'h'],
            vec![&'c', &'f', &'i'],
        ];

        let transposed = g.column_iter().collect::<Vec<_>>();

        assert_eq!(columns, transposed);
    }

    #[test]
    fn row_columns_iter() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 0), &'a'),
            ((0, 1), &'b'),
            ((0, 2), &'c'),
            ((1, 0), &'d'),
            ((1, 1), &'e'),
            ((1, 2), &'f'),
            ((2, 0), &'g'),
            ((2, 1), &'h'),
            ((2, 2), &'i'),
        ];

        assert_eq!(v, g.row_column_index_value_iter().collect::<Vec<_>>());
    }

    #[test]
    fn hv_neighbors_middle() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 1), HorizontalVerticalDirection::Up),
            ((1, 2), HorizontalVerticalDirection::Right),
            ((2, 1), HorizontalVerticalDirection::Down),
            ((1, 0), HorizontalVerticalDirection::Left),
        ];

        assert_eq!(v, g.hv_neighbors(1, 1));
    }

    #[test]
    fn hv_neighbors_corner() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 1), HorizontalVerticalDirection::Right),
            ((1, 0), HorizontalVerticalDirection::Down),
        ];

        assert_eq!(v, g.hv_neighbors(0, 0));
    }

    #[test]
    fn hvd_neighbors_middle() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 1), HorizontalVerticalDiagonalDirection::Up),
            ((0, 2), HorizontalVerticalDiagonalDirection::UpRight),
            ((1, 2), HorizontalVerticalDiagonalDirection::Right),
            ((2, 2), HorizontalVerticalDiagonalDirection::DownRight),
            ((2, 1), HorizontalVerticalDiagonalDirection::Down),
            ((2, 0), HorizontalVerticalDiagonalDirection::DownLeft),
            ((1, 0), HorizontalVerticalDiagonalDirection::Left),
            ((0, 0), HorizontalVerticalDiagonalDirection::UpLeft),
        ];

        assert_eq!(v, g.hvd_neighbors(1, 1));
    }

    #[test]
    fn hvd_neighbors_corner() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 1), HorizontalVerticalDiagonalDirection::Right),
            ((1, 1), HorizontalVerticalDiagonalDirection::DownRight),
            ((1, 0), HorizontalVerticalDiagonalDirection::Down),
        ];

        assert_eq!(v, g.hvd_neighbors(0, 0));
    }
}
