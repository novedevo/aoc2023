fn main() {
    let input = include_str!("../../data/day9.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = Map {
        inner: input,
        explored: vec![],
    };
    dbg!(map.product_top_three_basins());
}

struct Map {
    inner: Vec<Vec<u32>>,
    explored: Vec<Vec<bool>>,
}
impl Map {
    fn product_top_three_basins(&mut self) -> u32 {
        let (height, width) = (self.inner.len(), self.inner[0].len());
        self.explored = vec![vec![false; width]; height];
        let mut basin_sizes = vec![];
        for row in 0..height {
            for col in 0..width {
                if self.explored[row][col] || self.inner[row][col] == 9 {
                    continue;
                } else {
                    basin_sizes.push(self.size(row, col));
                }
            }
        }
        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        basin_sizes[..3].iter().product::<usize>() as u32
    }
    fn size(&mut self, row: usize, col: usize) -> usize {
        let map = &self.inner;
        let explored = &mut self.explored;
        let risk = map[row][col];
        if explored[row][col] || risk == 9 {
            return 0;
        }
        explored[row][col] = true;
        let (height, width) = (map.len(), map[0].len());
        let (top, bottom, left, right) = (row == 0, row == height - 1, col == 0, col == width - 1);

        1 + (if !top { self.size(row - 1, col) } else { 0 }
            + if !bottom { self.size(row + 1, col) } else { 0 }
            + if !left { self.size(row, col - 1) } else { 0 }
            + if !right { self.size(row, col + 1) } else { 0 })
    }
}
