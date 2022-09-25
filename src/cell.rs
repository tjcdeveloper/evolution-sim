pub struct Cell {
    contents: Option<Box<dyn CellContent>>,
    x: u16,
    y: u16
}

impl Cell {
    pub fn create(x: u16, y: u16) -> Self {
        Self {x, y, contents: None}
    }

    pub fn get_cell_key(col: u16, row: u16, num_cols: u16) -> u16 {
        num_cols * row + col
    }
}

pub trait CellContent {
    fn draw(&self);
}