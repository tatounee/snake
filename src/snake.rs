
mod test;

use rand::Rng;
use std::collections::VecDeque;
use std::fmt;

const BOARD_LEN: usize = 20;
const COORD_START: Coord = Coord(5, 15);

trait Utils<C> {
    fn get_last_coord(&self) -> &C;
    fn get_coord(&self, i: usize) -> &C;
    fn contains_(&self, coord: &Coord) -> bool;
}

impl Utils<Coord> for VecDeque<(Coord, ComplexDirection)> {
    #[inline]
    fn get_last_coord(&self) -> &Coord {
        &self.get(self.len() - 1).unwrap().0
    }

    #[inline]
    fn get_coord(&self, i: usize) -> &Coord {
        &self.get(i).unwrap().0
    }

    #[inline]
    fn contains_(&self, coord: &Coord) -> bool {
        self.iter().any(|t| &t.0 == coord)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

pub struct Board {
    pub board: [[Tile; BOARD_LEN]; BOARD_LEN],
    coords: VecDeque<(Coord, ComplexDirection)>,
    buffer_direction: VecDeque<SimpleDirection>,
    pub last_direction: SimpleDirection,
    apple: Coord,
}

impl Board {
    pub fn new() -> Self {
        let mut vecdeque_coords = VecDeque::new();
        vecdeque_coords.push_back((COORD_START, ComplexDirection::Right));
        Self {
            board: [[Tile::Empty; BOARD_LEN]; BOARD_LEN],
            coords: vecdeque_coords,
            buffer_direction: VecDeque::new(),
            last_direction: SimpleDirection::Right,
            apple: Coord(15, 5),
        }
    }

    pub fn get_direction(&self) -> &SimpleDirection {
        if self.buffer_direction.len() == 0 {
            &self.last_direction
        } else {
            &self
                .buffer_direction
                .get(self.buffer_direction.len() - 1)
                .unwrap()
        }
    }

    pub fn set_direction(&mut self, direction: SimpleDirection) {
        if self.buffer_direction.len() == 2 || self.get_direction() == &direction {
            return;
        }
        let cmp_direction = if self.buffer_direction.len() == 0 {
            &self.last_direction
        } else {
            &self
                .buffer_direction
                .get(self.buffer_direction.len() - 1)
                .unwrap()
        };
        match direction {
            SimpleDirection::Up => {
                if cmp_direction != &SimpleDirection::Down {
                    self.buffer_direction.push_back(direction);
                }
            }
            SimpleDirection::Down => {
                if cmp_direction != &SimpleDirection::Up {
                    self.buffer_direction.push_back(direction);
                }
            }
            SimpleDirection::Right => {
                if cmp_direction != &SimpleDirection::Left {
                    self.buffer_direction.push_back(direction);
                }
            }
            SimpleDirection::Left => {
                if cmp_direction != &SimpleDirection::Right {
                    self.buffer_direction.push_back(direction);
                }
            }
        }
    }

    pub fn grow_snake(&mut self) -> Option<bool> {
        if let Some(direction) = self.buffer_direction.pop_front() {
            println!("{:?} + {:?} = {:?}", self.last_direction, direction, ComplexDirection::from_two_direction(&self.last_direction, &direction));
            self.coords.get_mut(self.coords.len() - 1).unwrap().1 = ComplexDirection::from_two_direction(&self.last_direction, &direction);
            self.last_direction = direction;
        }
        match self.last_direction {
            SimpleDirection::Up => {
                if let Some(_) = self.board.get(self.coords.get_last_coord().1.wrapping_sub(1)) {
                    let coord = Coord(self.coords.get_last_coord().0, self.coords.get_last_coord().1 - 1);
                    if &coord == self.coords.get_coord(0) {
                        self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                        Some(false)
                    } else {
                        if !self.coords.contains_(&coord) {
                            let eat = coord == self.apple;
                            self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                            Some(eat)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            SimpleDirection::Down => {
                if let Some(_) = self.board.get(self.coords.get_last_coord().1 + 1) {
                    let coord = Coord(self.coords.get_last_coord().0, self.coords.get_last_coord().1 + 1);
                    if &coord == self.coords.get_coord(0) {
                        self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                        Some(false)
                    } else {
                        if !self.coords.contains_(&coord) {
                            let eat = coord == self.apple;
                            self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                            Some(eat)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            SimpleDirection::Right => {
                if let Some(_) = self
                    .board
                    .get(self.coords.get_last_coord().1)
                    .unwrap()
                    .get(self.coords.get_last_coord().0 + 1)
                {
                    let coord = Coord(self.coords.get_last_coord().0 + 1, self.coords.get_last_coord().1);
                    if &coord == self.coords.get_coord(0) {
                        self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                        Some(false)
                    } else {
                        if !self.coords.contains_(&coord) {
                            let eat = coord == self.apple;
                            self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                            Some(eat)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            SimpleDirection::Left => {
                if let Some(_) = self
                    .board
                    .get(self.coords.get_last_coord().1)
                    .unwrap()
                    .get(self.coords.get_last_coord().0.wrapping_sub(1))
                {
                    let coord = Coord(self.coords.get_last_coord().0 - 1, self.coords.get_last_coord().1);
                    if &coord == self.coords.get_coord(0) {
                        self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                        Some(false)
                    } else {
                        if !self.coords.contains_(&coord) {
                            let eat = coord == self.apple;
                            self.coords.push_back((coord, ComplexDirection::from_one_direction(self.last_direction)));
                            Some(eat)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn move_queue(&mut self) {
        self.coords.pop_front();
    }

    pub fn generate_apple(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let coord = Coord(rng.gen_range(0, BOARD_LEN), rng.gen_range(0, BOARD_LEN));
            if !self.coords.contains_(&coord) {
                self.apple = coord;
                break;
            }
        }
    }

    pub fn update_board(&mut self) {
        let mut new_board = [[Tile::Empty; BOARD_LEN]; BOARD_LEN];
        new_board[self.apple.1][self.apple.0] = Tile::Apple;
        
        for t in self.coords.iter().skip(1).take(self.coords.len() - 1) {
            new_board[t.0.1][t.0.0] = Tile::Body(t.1)
        }

        new_board[self.coords.get_last_coord().1][self.coords.get_last_coord().0] =
            Tile::Head(self.last_direction);
        new_board[self.coords[0].0.1][self.coords[0].0.0] =
                Tile::Queue(self.coords[0].1.into_direction());
        self.board = new_board;
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| {
                            match tile {
                                Tile::Empty => "-", // "⬜", //"▢"
                                Tile::Apple => "@", // "🟥",
                                Tile::Head(d) => match d {
                                    SimpleDirection::Up => "U",
                                    SimpleDirection::Down => "D",
                                    SimpleDirection::Right => "R",
                                    SimpleDirection::Left => "L",
                                } // "🟡",
                                Tile::Queue(d) => {
                                    match d {
                                        SimpleDirection::Up => "u",
                                        SimpleDirection::Down => "d",
                                        SimpleDirection::Right => "r",
                                        SimpleDirection::Left => "l",
                                    }
                                } 
                                Tile::Body(d) => {
                                    print!("{:?}", d); 
                                    match d {
                                        ComplexDirection::Up => "⭡", //  ⭠ ⭢ ⭡ ⭣ 
                                        ComplexDirection::Down => "⭣",
                                        ComplexDirection::Left => "⭠",
                                        ComplexDirection::Right => "⭢",
                                        ComplexDirection::UpLeft => "⭦", //  ⭦ ⭧ ⭨ ⭩ 
                                        ComplexDirection::UpRight => "⭧",
                                        ComplexDirection::DownLeft => "⭩",
                                        ComplexDirection::DownRight => "⭨",
                                        ComplexDirection::LeftUp => "⭦",
                                        ComplexDirection::LeftDown => "⭩",
                                        ComplexDirection::RightUp => "⭧",
                                        ComplexDirection::RightDown => "⭨",
                                    }  // "🟢",
                                }
                            }
                        })
                        .collect::<String>()
                        + "\n"
                })
                .collect::<String>()
        )
    }
}

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Apple,
    Head(SimpleDirection),
    Queue(SimpleDirection),
    Body(ComplexDirection),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SimpleDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone)]
pub enum ComplexDirection {
    Up,
    Down,
    Right,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown
}

impl ComplexDirection {
    fn from_one_direction(direction: SimpleDirection) -> Self {
        match direction {
            SimpleDirection::Up => Self::Up,
            SimpleDirection::Down => Self::Down,
            SimpleDirection::Left => Self::Left,
            SimpleDirection::Right => Self::Right,
        }
    }

    fn from_two_direction(from_direction: &SimpleDirection, to_direction: &SimpleDirection) -> Self {
        match from_direction {
            SimpleDirection::Up => {
                match to_direction {
                    SimpleDirection::Up => Self::Up,
                    SimpleDirection::Down => Self::Down,
                    SimpleDirection::Left => Self::UpLeft,
                    SimpleDirection::Right => Self::UpRight,
                }
            }
            SimpleDirection::Down => {
                match to_direction {
                    SimpleDirection::Up => Self::Up,
                    SimpleDirection::Down => Self::Down,
                    SimpleDirection::Left => Self::DownLeft,
                    SimpleDirection::Right => Self::DownRight,
                }
            }
            SimpleDirection::Left => {
                match to_direction {
                    SimpleDirection::Up => Self::LeftUp,
                    SimpleDirection::Down => Self::LeftDown,
                    SimpleDirection::Left => Self::Left,
                    SimpleDirection::Right => Self::Right,
                }
            }
            SimpleDirection::Right => {
                match to_direction {
                    SimpleDirection::Up => Self::RightUp,
                    SimpleDirection::Down => Self::RightDown,
                    SimpleDirection::Left => Self::Left,
                    SimpleDirection::Right => Self::Right,
                }
            }
        }
    }

    fn into_direction(&self) -> SimpleDirection {
        match self {
            Self::Up => SimpleDirection::Up,
            Self::Down => SimpleDirection::Down,
            Self::Right => SimpleDirection::Right,
            Self::Left => SimpleDirection::Left,
            Self::UpLeft => SimpleDirection::Left,
            Self::UpRight => SimpleDirection::Right,
            Self::DownLeft => SimpleDirection::Left,
            Self::DownRight => SimpleDirection::Right,
            Self::LeftUp => SimpleDirection::Up,
            Self::LeftDown => SimpleDirection::Down,
            Self::RightUp => SimpleDirection::Up,
            Self::RightDown => SimpleDirection::Down,
        }
    }
}

pub struct Timer {
    pub time: f64,
    pub time_since_last_eat: f64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: 0.,
            time_since_last_eat: 0.,
        }
    }

    #[inline]
    pub fn add(&mut self, time: f64) {
        self.time += time;
        self.time_since_last_eat += time;
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut time = self.time as u64;
        let hours = time / 3600;
        time %= 3600;
        let minutes = time / 60;
        let seconds = time % 60;
        if hours == 0 {
            write!(f, "{}:{:0>2}", minutes, seconds)
        } else {
            write!(f, "{}:{}:{:0>2}", hours, minutes, seconds)
        }
    }
}

