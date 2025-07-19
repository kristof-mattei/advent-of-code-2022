use std::ops::{Deref, DerefMut, Index};

use super::{
    GridIter, HorizontalVerticalDiagonalDirection, HorizontalVerticalDirection, Neighbors,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InfiniteRow<T>(Vec<T>);

impl<T: Clone> std::clone::Clone for InfiniteRow<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Index<isize> for InfiniteRow<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let row_index = index
            .rem_euclid(self.0.len().try_into().expect("Row too long"))
            .unsigned_abs();

        &self.0[row_index]
    }
}

impl<T> Index<usize> for InfiniteRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> Deref for InfiniteRow<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> DerefMut for InfiniteRow<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }
}

pub struct InfiniteGrid<T> {
    data: Vec<InfiniteRow<T>>,
    row_len: usize,
    column_len: usize,
    // max_row: usize,
    // max_column: usize,
}

impl<T> Deref for InfiniteGrid<T> {
    type Target = [InfiniteRow<T>];

    fn deref(&self) -> &[InfiniteRow<T>] {
        self.data.as_slice()
    }
}

impl<T> DerefMut for InfiniteGrid<T> {
    fn deref_mut(&mut self) -> &mut [InfiniteRow<T>] {
        self.data.as_mut_slice()
    }
}

impl<T: Clone> Clone for InfiniteGrid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            row_len: self.row_len,
            column_len: self.column_len,
        }
    }
}

impl<T> InfiniteGrid<T> {
    /// Creates a grid that repeats infinitely in each direction repeating on the data
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
            data: data.into_iter().map(|r| InfiniteRow(r)).collect(),
            row_len: rows,
            column_len: columns,
            // max_row: rows - 1,
            // max_column: columns - 1,
        }
    }
}

impl<T> GridIter for InfiniteGrid<T> {
    type GridRow = InfiniteRow<T>;

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

impl<T> Neighbors for InfiniteGrid<T> {
    type Index = isize;

    fn hv_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> Vec<((Self::Index, Self::Index), HorizontalVerticalDirection)> {
        vec![
            (
                (row_index - 1, column_index),
                HorizontalVerticalDirection::Up,
            ),
            (
                (row_index, column_index + 1),
                HorizontalVerticalDirection::Right,
            ),
            (
                (row_index + 1, column_index),
                HorizontalVerticalDirection::Down,
            ),
            (
                (row_index, column_index - 1),
                HorizontalVerticalDirection::Left,
            ),
        ]
    }

    fn hvd_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> Vec<(
        (Self::Index, Self::Index),
        HorizontalVerticalDiagonalDirection,
    )> {
        vec![
            (
                (row_index - 1, column_index),
                HorizontalVerticalDiagonalDirection::Up,
            ),
            (
                (row_index - 1, column_index + 1),
                HorizontalVerticalDiagonalDirection::UpRight,
            ),
            (
                (row_index, column_index + 1),
                HorizontalVerticalDiagonalDirection::Right,
            ),
            (
                (row_index + 1, column_index + 1),
                HorizontalVerticalDiagonalDirection::DownRight,
            ),
            (
                (row_index + 1, column_index),
                HorizontalVerticalDiagonalDirection::Down,
            ),
            (
                (row_index + 1, column_index - 1),
                HorizontalVerticalDiagonalDirection::DownLeft,
            ),
            (
                (row_index, column_index - 1),
                HorizontalVerticalDiagonalDirection::Left,
            ),
            (
                (row_index - 1, column_index - 1),
                HorizontalVerticalDiagonalDirection::UpLeft,
            ),
        ]
    }
}

impl<T> std::fmt::Display for InfiniteGrid<T>
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

impl<T> std::fmt::Debug for InfiniteGrid<T>
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

impl<T> Index<usize> for InfiniteGrid<T> {
    type Output = InfiniteRow<T>;

    fn index(&self, index: usize) -> &Self::Output {
        let row_index = index.rem_euclid(self.row_len);

        &self.data[row_index]
    }
}

impl<T> Index<isize> for InfiniteGrid<T> {
    type Output = InfiniteRow<T>;

    fn index(&self, index: isize) -> &Self::Output {
        let row_index = index
            .rem_euclid(self.row_len.try_into().expect("row_count too large"))
            .unsigned_abs();

        &self.data[row_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::grids::infinite_grid::InfiniteGrid;
    use crate::shared::grids::{
        HorizontalVerticalDiagonalDirection, HorizontalVerticalDirection, Neighbors as _,
    };

    #[test]
    fn infinite_grid() {
        let g = InfiniteGrid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        assert_eq!('b', g[-9_isize][-5_isize]);

        assert_eq!('i', g[8_isize][8_isize]);
    }

    #[test]
    fn hv_neighbors_middle() {
        let g = InfiniteGrid::new(vec![
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
        let g = InfiniteGrid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((-1, 0), HorizontalVerticalDirection::Up),
            ((0, 1), HorizontalVerticalDirection::Right),
            ((1, 0), HorizontalVerticalDirection::Down),
            ((0, -1), HorizontalVerticalDirection::Left),
        ];

        assert_eq!(v, g.hv_neighbors(0, 0));
    }

    #[test]
    fn hvd_neighbors_middle() {
        let g = InfiniteGrid::new(vec![
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
        let g = InfiniteGrid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((-1, 0), HorizontalVerticalDiagonalDirection::Up),
            ((-1, 1), HorizontalVerticalDiagonalDirection::UpRight),
            ((0, 1), HorizontalVerticalDiagonalDirection::Right),
            ((1, 1), HorizontalVerticalDiagonalDirection::DownRight),
            ((1, 0), HorizontalVerticalDiagonalDirection::Down),
            ((1, -1), HorizontalVerticalDiagonalDirection::DownLeft),
            ((0, -1), HorizontalVerticalDiagonalDirection::Left),
            ((-1, -1), HorizontalVerticalDiagonalDirection::UpLeft),
        ];

        assert_eq!(v, g.hvd_neighbors(0, 0));
    }
}
