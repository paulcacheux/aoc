#[derive(Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
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

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, val)| (i % self.width, i / self.width, val))
    }

    #[inline]
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::with_capacity(4);
        if x != 0 {
            n.push((x - 1, y));
        }
        if y != 0 {
            n.push((x, y - 1));
        }
        if x != self.width - 1 {
            n.push((x + 1, y));
        }
        if y != self.height - 1 {
            n.push((x, y + 1));
        }
        n
    }
}
