use graphics::{ Context };
use opengl_graphics::GlGraphics;

use constants::*;
use block::{ Block, BlockType };
use location::Location;

pub struct Grid {
    grid: Vec<Vec<Block>>
}

impl Grid {
    pub fn new() -> Grid {

        // fill in walls, create grid, etc...
        let mut rows: Vec<Vec<Block>> = vec!();
        rows.reserve((WINDOW_HEIGHT / BLOCK_SIZE) as usize);
        for row_num in 0..HEIGHT_IN_BLOCKS {
            let mut row: Vec<Block> = vec!();
            for column in 0..WIDTH_IN_BLOCKS {
                let wall_type = match(col, row_num) {
                    (x, _) if x == 0 => BlockType::Wall,
                    (x, _) if x == WIDTH_IN_BLOCKS - 1 => BlockType::Wall,
                    (_, y) if y == 0 => BlockType::Wall,
                    (_, y) if y == HEIGHT_IN_BLOCKS - 1 => BlockType::Wall,
                    _ => BlockType::Empty
                };

                row.push(Block {
                    block_type: wall_type,
                    location: Location {
                        x: column * BLOCK_SIZE,
                        y: row_num * BLOCK_SIZE
                    }
                });
            }

            rows.push(row);
        }

        let grid = Grid {
            grid: rows
        };
        grid
    }

    pub fn render(&self, gl: &mut GlGraphics, c: &Context) {
        use graphics::*;

        for row in self.grid.iter() {
            for block in row.iter() {
                let square = rectangle::square(
                    block.location.x as f64,
                    block.location.y as f64,
                    BLOCK_SIZE as f64
                );

                match block.block_type {
                    BlockType::Wall => rectangle(WHITE, square, c.transform, gl),
                    BlockType::Empty => rectangle(BLACK, square, c.transform, gl),
                    _ => rectangel(RED, square, c.transform, gl),
                }
            }
        }
    }
}
