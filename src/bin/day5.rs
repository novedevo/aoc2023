fn parse_usize(s: &str) -> usize {
    s.parse().unwrap()
}

#[derive(Clone, Copy)]
struct Direction {
    startx: usize,
    starty: usize,
    endx: usize,
    endy: usize,
}
impl Direction {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();
        let ((startx, starty), (endx, endy)) =
            (start.split_once(',').unwrap(), end.split_once(',').unwrap());
        Self {
            startx: parse_usize(startx),
            starty: parse_usize(starty),
            endx: parse_usize(endx),
            endy: parse_usize(endy),
        }
    }
    fn is_orthogonal(&self) -> bool {
        self.startx == self.endx || self.starty == self.endy
    }
    fn is_vertical(&self) -> bool {
        self.startx == self.endx
    }
}

fn main() {
    let input = include_str!("../../data/day5_test.txt")
        .lines()
        .map(Direction::from_str);

    let (orthogonal, diagonal): (Vec<Direction>, Vec<Direction>) =
        input.partition(Direction::is_orthogonal);
    let (vertical, horizontal): (Vec<Direction>, Vec<Direction>) =
        orthogonal.iter().partition(|dir| dir.is_vertical());

    let mut log = vec![[0; 10]; 10];
    for direction in vertical {
        let range = if direction.starty > direction.endy {
            direction.endy..=direction.starty
        } else {
            direction.starty..=direction.endy
        };
        for y in range {
            log[y][direction.startx] += 1;
        }
    }
    for direction in horizontal {
        let range = if direction.startx > direction.endx {
            direction.endx..=direction.startx
        } else {
            direction.startx..=direction.endx
        };
        for x in range {
            log[direction.starty][x] += 1;
        }
    }
    for direction in diagonal {
        let length = (direction.startx as i32 - direction.endx as i32).abs();
        let (x_inc, y_inc) = (
            if direction.startx > direction.endx {
                -1
            } else {
                1
            },
            if direction.starty > direction.endy {
                -1
            } else {
                1
            },
        );
        for i in 0..=length {
            log[(direction.starty as i32 + i * y_inc) as usize]
                [(direction.startx as i32 + i * x_inc) as usize] += 1;
        }
    }
    let intersections = log
        .into_iter()
        .map(|row| {
            row.into_iter()
                .fold(0, |acc, current| if current > 1 { acc + 1 } else { acc })
        })
        .reduce(|acc, current| acc + current)
        .unwrap();

    println!("{}", intersections)
}
