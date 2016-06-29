use bike::*;

/// Grid struct. Knows about itself and the two bikes. ********************************************

pub struct Grid {
    pub x_max: i16,
    pub x_min: i16,
    pub y_max: i16,
    pub y_min: i16,
    pub trails: Vec<(i16, i16)>,
}

impl Grid {
    pub fn new(x_max: i16, x_min: i16, y_max: i16, y_min: i16) -> Grid {
        let mut initial_trails = Vec::new();

        initial_trails.push((x_min, y_max / 2));
        initial_trails.push((x_max, y_max / 2));

        Grid {
            x_max: x_max,
            x_min: x_min,
            y_max: y_max,
            y_min: y_min,
            trails: initial_trails,
        }
    }

    pub fn collect_bike_one_trail(&mut self, bike_one: &Bike) {
        self.trails.push((bike_one.x, bike_one.y));
    }

    pub fn collect_bike_two_trail(&mut self, bike_two: &Bike) {
        self.trails.push((bike_two.x, bike_two.y));
    }
}

#[test]
fn it_intializes_properly() {
    let grid = Grid::new(800, 0, 800 ,0);

    assert_eq!(grid.x_max, 800);
    assert_eq!(grid.x_min, 0);
    assert_eq!(grid.y_max, 800);
    assert_eq!(grid.y_min, 0);
}

#[test]
fn it_can_collect_bike_trails() {
    let mut bike_one = Bike::new(0, 400);
    let mut bike_two = Bike::new(0, 400);
    let mut grid = Grid::new(800, 0, 800 ,0);

    assert_eq!(grid.trails[0], (0, 400));
    assert_eq!(grid.trails[1], (800, 400));

    bike_one.move_right();
    grid.collect_bike_one_trail(&bike_one);

    bike_two.move_right();
    grid.collect_bike_two_trail(&bike_two);

    assert_eq!(grid.trails[2], (1, 400));
    assert_eq!(grid.trails[3], (1, 400));
}