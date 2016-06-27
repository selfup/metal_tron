#[allow(unused_variables)]
fn main() {
    let grid = Grid::new();
    let mut bike_one = Bike::new();

    bike_one.move_right();
    bike_one.status_check(&grid);
    bike_one.print_properties();

    for i in 0..802 {
        bike_one.move_right();
        bike_one.status_check(&grid);
    }

    bike_one.move_left();
    bike_one.status_check(&grid);
    bike_one.print_properties();
}

struct Grid {
    x_max: i16,
    x_min: i16,
    y_max: i16,
    y_min: i16,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            x_max: 800,
            x_min: 0,
            y_max: 800,
            y_min: 0,
        }
    }
}

struct Bike {
    x: i16,
    y: i16,
    status: &'static str,
}

impl Bike {
    fn new() -> Bike {
        Bike {
            x: 0,
            y: 0,
            status: "alive",
        }
    }

    fn print_properties(&self) {
        println!("Bike coords: x -> {}, y -> {}, Bike status: {}",
                 self.x,
                 self.y,
                 self.status);
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn status_check(&mut self, grid: &Grid) {
        if self.x > grid.x_max {
            self.x = grid.x_min;
        } else if self.y > grid.y_max {
            self.y = grid.y_min;
        } else if self.x < grid.x_min {
            self.x = grid.x_max;
        } else if self.y < grid.y_min {
            self.y = grid.y_max;
        }
    }
}