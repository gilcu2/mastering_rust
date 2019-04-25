#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human,
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    beings: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}

impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square { ground: TerrainGround::Soil, block: None, beings: None });
        }

        Grid {
            size: (size_x, size_y),
            squares: squares,
        }
    }
}

#[cfg(test)]
mod tests {
    //! tests of the game
    use crate::*;

    #[test]
    fn test_empty_grid() {
        //! Given an initial grid
        let x_size=5;
        let y_size=13;
        let grid = Grid::generate_empty(x_size, y_size);

        assert_eq!(grid.size, (x_size, y_size));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.beings, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), x_size * y_size);
        assert_eq!(number_of_squares, x_size * y_size);
    }
}