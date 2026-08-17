#[derive(Debug, PartialEq, Eq)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    cells: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new(cells: [[T; COLS]; ROWS]) -> Self {
        Self { cells }
    }

    fn dimensions(&self) -> (usize, usize) {
        (ROWS, COLS)
    }
}

fn main() {
    let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(matrix.dimensions(), (2, 3));
    assert_eq!(matrix.cells[1][2], 6);
}
