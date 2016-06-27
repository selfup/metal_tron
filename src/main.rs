#[allow(unused_variables)]
fn main() {
    let mut game = Game::new(Grid::new(800, 800), Bike::new(0, 0));

    game.bike_one.move_right();
    game.bike_one.status_check(&game.grid);
    game.bike_one.print_properties();

    for i in 0..802 {
        game.bike_one.move_right();
        game.bike_one.status_check(&game.grid);
    }

    game.bike_one.move_left();
    game.bike_one.status_check(&game.grid);
    game.bike_one.print_properties();
}

struct Game {
    grid: Grid,
    bike_one: Bike,
}

impl Game {
    fn new(grid: Grid, bike_one: Bike) -> Game {
        Game {
            grid: grid,
            bike_one: bike_one,
        }
    }
}

struct Grid {
    x_max: i16,
    x_min: i16,
    y_max: i16,
    y_min: i16,
}

impl Grid {
    fn new(x_max: i16, y_max: i16) -> Grid {
        Grid {
            x_max: x_max,
            x_min: 0,
            y_max: y_max,
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
    fn new(x: i16, y: i16) -> Bike {
        Bike {
            x: x,
            y: y,
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