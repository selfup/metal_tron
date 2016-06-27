#[allow(unused_variables)]
fn main() {
    let grid = Grid::new();

    let mut bike_one = Bike::new();
    
    bike_one.print_properties();
    bike_one.move_right();
    bike_one.print_properties();

    for i in 0..800 {
        bike_one.x += 1;
    }

    bike_one.status_check(grid);
    bike_one.print_properties();
}

struct Grid {
    x_limit: i16,
    y_limit: i16,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            x_limit: 800,
            y_limit: 800,
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

    fn status_check(&mut self, grid: Grid) {
        if self.x >= grid.x_limit {
            self.x = 0;
        } else if self.y >= grid.y_limit {
            self.y = 0;
        }
    }
}
