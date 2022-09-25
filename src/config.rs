pub struct Config {
    width: u32,
    height: u32,
}

impl Config {
    pub fn create(width: u32, height: u32) -> Self {
        Config {width, height}
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}