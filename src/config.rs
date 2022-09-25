pub struct Config {
    cell_height: u32,
    cell_width: u32,
    debug: bool,
    height: u32,
    num_cells_x: u32,
    num_cells_y: u32,
    width: u32,
}

impl Config {
    pub fn create(width: u32, height: u32, num_cells_x: u32, num_cells_y: u32, debug: bool) -> Self {
        let cell_width = width/num_cells_x;
        let cell_height = height/num_cells_y;
        Config {
            cell_height, 
            cell_width, 
            debug,
            height: num_cells_y * cell_height, 
            num_cells_x, 
            num_cells_y,
            width: num_cells_x * cell_width,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_cell_width(&self) -> u32 {
        self.cell_width
    }

    pub fn get_cell_height(&self) -> u32 {
        self.cell_height
    }

    pub fn get_num_cells_x(&self) -> u32 {
        self.num_cells_x
    }

    pub fn get_num_cells_y(&self) -> u32 {
        self.num_cells_y
    }   

    pub fn is_debug(&self) -> bool {
        self.debug
    }
}