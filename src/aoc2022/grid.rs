#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Grid {
            data: vec![value; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<F: Fn(char) -> T>(input: &str, mapper: F) -> Self {
        let mut heights = Vec::new();
        let mut width = None;
        let mut height = 0;

        for line in input.lines() {
            let line = line.trim();
            if let Some(w) = width {
                assert_eq!(w, line.len());
            } else {
                width = Some(line.len());
            }
            // heights.extend(line.chars().map(|c| c.to_string().parse::<u8>().unwrap()));
            heights.extend(line.chars().map(&mapper));
            height += 1;
        }

        Grid {
            data: heights,
            width: width.unwrap_or_default(),
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.width * y + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.width * y + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[self.width * y + x] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, val)| (i % self.width, i / self.width, val))
    }

    #[inline]
    pub fn get_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        self.get_neighbors_with_direction(x, y).map(|(_, x, y)| (x, y))
    }

    #[inline]
    pub fn get_neighbors_with_direction(&self, x: usize, y: usize) -> impl Iterator<Item = (Direction, usize, usize)> {
        let width = self.width;
        let height = self.height;
        std::iter::from_coroutine(move || {
            if x != 0 {
                yield (Direction::West, x - 1, y);
            }
            if y != 0 {
                yield (Direction::North, x, y - 1);
            }
            if x != width - 1 {
                yield (Direction::East, x + 1, y);
            }
            if y != height - 1 {
                yield (Direction::South, x, y + 1);
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}