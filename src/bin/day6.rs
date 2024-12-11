use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
enum Tile {
    Guard(Direction),
    Obstacle,
    Empty,
    Visited,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard(Direction::North),
            _ => unreachable!("Unknown type of tile"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    curr_tile: (i32, i32),
    out_of_map: bool,
}

#[allow(dead_code)]
impl Map {
    fn search_guard(&mut self) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                match self.tiles[i][j] {
                    Tile::Guard(_) => {
                        self.curr_tile = (i as i32, j as i32);
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }

    fn change_guard_direction(&mut self) {
        let x = self.curr_tile.0 as usize;
        let y = self.curr_tile.1 as usize;

        let tile = self.tiles[x][y];
        match tile {
            Tile::Guard(direction) => match direction {
                Direction::North => self.tiles[x][y] = Tile::Guard(Direction::East),
                Direction::East => self.tiles[x][y] = Tile::Guard(Direction::South),
                Direction::West => self.tiles[x][y] = Tile::Guard(Direction::North),
                Direction::South => self.tiles[x][y] = Tile::Guard(Direction::West),
            },
            _ => unreachable!("`curr_tile` is always supposed to be guard tile"),
        }
    }

    fn valid_tile(&self, i: usize, j: usize) -> bool {
        i < self.tiles.len() && j < self.tiles[0].len()
    }

    fn out_of_map(&self) -> bool {
        self.out_of_map
    }

    fn tile_is_obstacle(&self, i: usize, j: usize) -> bool {
        match self.tiles[i][j] {
            Tile::Obstacle => return true,
            Tile::Empty => return false,
            Tile::Visited => return false,
            _ => unreachable!("How can this be guard?"),
        };
    }

    fn travel(&mut self) {
        let i = self.curr_tile.0;
        let j = self.curr_tile.1;
        let (mut i, mut j, mut curr_direction) = match self.tiles[i as usize][j as usize] {
            Tile::Guard(direction) => match direction {
                Direction::North => (i - 1, j, direction),
                Direction::East => (i, j + 1, direction),
                Direction::West => (i, j - 1, direction),
                Direction::South => (i + 1, j, direction),
            },
            _ => unreachable!("`curr_tile` is always supposed to be guard tile"),
        };

        while self.valid_tile(i as usize, j as usize)
            && !self.tile_is_obstacle(i as usize, j as usize)
        {
            self.tiles[self.curr_tile.0 as usize][self.curr_tile.1 as usize] = Tile::Visited;
            self.curr_tile.0 = i as i32;
            self.curr_tile.1 = j as i32;
            self.tiles[self.curr_tile.0 as usize][self.curr_tile.1 as usize] =
                Tile::Guard(curr_direction);

            (i, j, curr_direction) =
                match self.tiles[self.curr_tile.0 as usize][self.curr_tile.1 as usize] {
                    Tile::Guard(direction) => match direction {
                        Direction::North => (i - 1, j, direction),
                        Direction::East => (i, j + 1, direction),
                        Direction::West => (i, j - 1, direction),
                        Direction::South => (i + 1, j, direction),
                    },
                    _ => unreachable!("`curr_tile` is always supposed to be guard tile"),
                };
        }

        if !self.valid_tile(i as usize, j as usize) {
            self.tiles[self.curr_tile.0 as usize][self.curr_tile.1 as usize] = Tile::Visited;
            self.curr_tile.0 = i as i32;
            self.curr_tile.1 = j as i32;
            self.out_of_map = true;
        }
    }

    fn count_visited(&self) -> usize {
        let mut visited = 0;

        for line in &self.tiles {
            for tile in line {
                match tile {
                    Tile::Visited => visited += 1,
                    _ => continue,
                }
            }
        }

        visited
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut tiles = vec![];
        for line in value.lines() {
            tiles.push(line.chars().map(|ch| Tile::from(ch)).collect());
        }

        Self {
            tiles,
            curr_tile: (0, 0),
            out_of_map: false,
        }
    }
}

fn parta(input: &str) {
    let mut map = Map::from(input);
    map.search_guard();

    while !map.out_of_map() {
        map.travel();
        if map.valid_tile(map.curr_tile.0 as usize, map.curr_tile.1 as usize) {
            map.change_guard_direction();
        }
    }

    let visited = map.count_visited();
    println!("Visited count: {visited}");
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/6.txt").unwrap().read_to_string(&mut str);

    parta(&str);
}
