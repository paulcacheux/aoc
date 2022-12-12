#[derive(Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
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
            heights.extend(line.chars().map(|c| c.to_string().parse::<u8>().unwrap()));
            height += 1;
        }

        Grid {
            data: heights,
            width: width.unwrap_or_default(),
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.width * y + x]
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, val)| (i % self.width, i / self.width, val))
    }
}
