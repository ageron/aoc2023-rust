use pathfinding::directed::astar::astar;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Crucible {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    num_straight: i32,
}

impl Crucible {
    fn successors(
        &self,
        heat_loss_map: &[&[u8]],
        width: i32,
        height: i32,
        is_ultra: bool,
    ) -> Vec<(Crucible, i32)> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|&(dx, dy)| {
                if self.dx == -dx && self.dy == -dy {
                    return None;
                }
                let is_straight = self.dx == dx && self.dy == dy;
                let num_straight = if is_straight {
                    self.num_straight + 1
                } else {
                    1
                };
                if is_ultra {
                    if num_straight > 10 {
                        return None;
                    }
                    if (self.dx != 0 || self.dy != 0) && !is_straight && self.num_straight < 4 {
                        return None;
                    }
                } else if num_straight > 3 {
                    return None;
                }
                let x = self.x + dx;
                let y = self.y + dy;
                if x < 0 || x >= width || y < 0 || y >= height {
                    return None;
                }
                let heat_loss = (heat_loss_map[y as usize][x as usize] - b'0') as i32;
                Some((
                    Crucible {
                        x,
                        y,
                        dx,
                        dy,
                        num_straight,
                    },
                    heat_loss,
                ))
            })
            .collect()
    }
    fn heuristic(&self, width: i32, height: i32, is_ultra: bool) -> i32 {
        (self.x - width + 1).abs()
            + (self.y - height + 1)
                .abs()
                .max(if is_ultra { 4 - self.num_straight } else { 0 })
    }
    fn success(&self, width: i32, height: i32, is_ultra: bool) -> bool {
        self.x == width - 1 && self.y == height - 1 && (!is_ultra || self.num_straight >= 4)
    }
}

fn find_min_heat_loss(heat_loss_map: &[&[u8]], is_ultra: bool) -> i32 {
    let width = heat_loss_map[0].len() as i32;
    let height = heat_loss_map[0].len() as i32;
    let (_, total_heat_loss) = astar(
        &Crucible::default(),
        |n| n.successors(heat_loss_map, width, height, is_ultra),
        |n| n.heuristic(width, height, is_ultra),
        |n| n.success(width, height, is_ultra),
    )
    .unwrap();
    total_heat_loss
}

pub fn run(input: &str) {
    let heat_loss_map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    for is_ultra in [false, true] {
        let min_heat_loss = find_min_heat_loss(&heat_loss_map, is_ultra);
        println!("{min_heat_loss}");
    }
}
