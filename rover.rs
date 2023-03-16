type Direction = u8;
type Coord = u32;

const NORTH: Direction = 0;
const EAST: Direction = 1;
const SOUTH: Direction = 2;
const WEST: Direction = 3;

struct Rover<'bounds> {
    x: Coord,
    y: Coord,
    d: Direction,
    bounds: &'bounds Bounds,
}

impl<'rover> Rover<'rover> {
    const fn new(x: Coord, y: Coord, d: Direction, bounds: &'rover Bounds) -> Self {
        Self { x, y, d, bounds }
    }

    fn left(self: Self) -> Self {
        return Self {
            d: (self.d + (4 - 1)) % 4,
            ..self
        };
    }

    fn right(self: Self) -> Self {
        return Self {
            d: (self.d + (1)) % 4,
            ..self
        };
    }

    fn forward(self: Self) -> Self {
        let mut new_x = self.x;
        let mut new_y = self.y;
        match self.d {
            NORTH => new_y += 1,
            SOUTH => new_y += self.bounds.y - 1,
            EAST => new_x += 1,
            WEST => new_x += self.bounds.x - 1,
            _ => {}
        }
        return Self {
            x: new_x % self.bounds.x,
            y: new_y % self.bounds.y,
            ..self
        };
    }
}

struct Bounds {
    x: u32,
    y: u32,
}

impl Bounds {
    const fn new(x: u32, y: u32) -> Self {
        return Self { x, y };
    }
}

mod tests {
    use crate::{Bounds, Rover, NORTH};

    const BOUNDS: Bounds = Bounds::new(10, 10);
    const INITIAL_ROVER: Rover = Rover::new(0, 0, NORTH, &BOUNDS);

    #[test]
    fn turning_left_four_times() {
        assert_eq!(NORTH, INITIAL_ROVER.left().left().left().left().d);
    }

    #[test]
    fn turning_right_four_times() {
        assert_eq!(NORTH, INITIAL_ROVER.right().right().right().right().d);
    }

    #[test]
    fn test_move_forward() {
        let initial: Rover = INITIAL_ROVER;
        let rover = initial.forward();
        assert_eq!(0, rover.x);
        assert_eq!(1, rover.y);
    }

    #[test]
    fn test_move_in_circle_left() {
        let initial: Rover = INITIAL_ROVER;
        let rover = initial
            .forward()
            .left()
            .forward()
            .left()
            .forward()
            .left()
            .forward()
            .left();
        assert_eq!(0, rover.x);
        assert_eq!(0, rover.y);
        assert_eq!(NORTH, rover.d);
    }

    #[test]
    fn test_move_in_circle_right() {
        let initial: Rover = INITIAL_ROVER;
        let rover = initial
            .forward()
            .right()
            .forward()
            .right()
            .forward()
            .right()
            .forward()
            .right();
        assert_eq!(0, rover.x);
        assert_eq!(0, rover.y);
        assert_eq!(NORTH, rover.d);
    }

    #[test]
    fn test_move_across_bounds() {
        let bounds = Bounds::new(2, 2);
        let initial: Rover = Rover::new(0, 0, NORTH, &bounds);
        let rover = initial.forward().forward();
        assert_eq!(0, rover.x);
        assert_eq!(0, rover.y);
    }

    #[test]
    fn test_move_across_bounds_south() {
        let bounds = Bounds::new(2, 2);
        let initial: Rover = Rover::new(0, 0, NORTH, &bounds);
        let rover = initial.left().left().forward().forward();
        assert_eq!(0, rover.x);
        assert_eq!(0, rover.y);
    }
}
