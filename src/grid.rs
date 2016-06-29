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