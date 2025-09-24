pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Vec<char>>, // 'W' for wall, '.' for floor
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = vec![vec!['.'; width as usize]; height as usize];

        for x in 0..width {
            tiles[0][x as usize] = 'W';
            tiles[height as usize - 1][x as usize] = 'W';
        }

        for y in 0..height {
            tiles[y as usize][0] = 'W';
            tiles[y as usize][width as usize - 1] = 'W';
        }

        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return true;
        }
        self.tiles[y as usize][x as usize] == 'W'
    }

    pub fn get_tiles(&self) -> &Vec<Vec<char>> {
        &self.tiles
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}
