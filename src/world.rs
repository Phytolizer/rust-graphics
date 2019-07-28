use crate::tile::{Tile, TileId};

pub(crate) struct World {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let mut tiles = Vec::with_capacity(width);
        for _ in 0..width {
            let mut col = Vec::with_capacity(height);
            for _ in 0..height {
                col.push(Tile::new());
            }
            tiles.push(col);
        }
        World {
            width,
            height,
            tiles,
        }
    }
    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, String> {
        if x >= self.width || y >= self.height {
            Err(format!("The tile at ({}, {}) is out of bounds", x, y))
        } else {
            Ok(&self.tiles[x][y])
        }
    }
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Result<&mut Tile, String> {
        if x >= self.width || y >= self.height {
            Err(format!("The tile at ({}, {}) is out of bounds", x, y))
        } else {
            Ok(&mut self.tiles[x][y])
        }
    }
    pub fn get_neighbors_to(&self, x: usize, y: usize) -> Result<[Option<TileId>; 4], String> {
        let mut neighbors: [Option<TileId>; 4] = [None; 4];
        // values cannot be < 0 as they are of type usize
        if x >= self.width || y >= self.height {
            Err(format!("The tile at ({}, {}) is out of bounds", x, y))
        } else {
            neighbors[0] = (if x == 0 {
                None
            } else {
                Some(self.tiles[x - 1][y].get_id())
            });
            neighbors[1] = (if y + 1 >= self.height {
                None
            } else {
                Some(self.tiles[x][y + 1].get_id())
            });
            neighbors[2] = (if x + 1 >= self.width {
                None
            } else {
                Some(self.tiles[x + 1][y].get_id())
            });
            neighbors[3] = (if y == 0 {
                None
            } else {
                Some(self.tiles[x][y - 1].get_id())
            });
            Ok(neighbors)
        }
    }
    pub fn update_cached_neighbors(&mut self) {
        for i in 0usize..self.width {
            for j in 0usize..self.height {
                let neighbors = self.get_neighbors_to(i, j).unwrap();
                let t = &mut self.tiles[i][j];
                t.set_neighbors(neighbors);
            }
        }
    }
}