#[allow(unused_variables)]
fn main() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400, "bike_one".to_string()),
                             Bike::new(800, 400, "bike_two".to_string()));

    game.bike_one_right();
    game.bike_one_left();

    game.bike_two_left();
    game.bike_two_right();

}

struct Game {
    grid: Grid,
    bike_one: Bike,
    bike_two: Bike,
}

impl Game {
    fn new(grid: Grid, bike_one: Bike, bike_two: Bike) -> Game {
        Game {
            grid: grid,
            bike_one: bike_one,
            bike_two: bike_two,
        }
    }

    fn bike_one_right(&mut self) {
        self.bike_one.move_right();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.print_properties();
    }

    fn bike_one_left(&mut self) {
        self.bike_one.move_left();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.print_properties();
    }

    fn bike_two_right(&mut self) {
        self.bike_two.move_right();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_two.print_properties();
    }

    fn bike_two_left(&mut self) {
        self.bike_two.move_left();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_two.print_properties();
    }
}

struct Grid {
    x_max: i16,
    x_min: i16,
    y_max: i16,
    y_min: i16,
    trails: Vec<(i16, i16)>,
}

impl Grid {
    fn new(x_max: i16, x_min: i16, y_max: i16, y_min: i16) -> Grid {
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

    fn collect_bike_one_trail(&mut self, bike_one: &Bike) {
        self.trails.push((bike_one.x, bike_one.y));
    }
}

struct Bike {
    x: i16,
    y: i16,
    name: String,
    status: String,
}

impl Bike {
    fn new(x: i16, y: i16, name: String) -> Bike {
        Bike {
            x: x,
            y: y,
            name: name,
            status: "alive".to_string(),
        }
    }

    fn print_properties(&self) {
        println!("{name} coords: x -> {}, y -> {}, {name} status: {}",
                 self.x,
                 self.y,
                 self.status,
                 name = self.name);
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn grid_check(&mut self, grid: &Grid) {
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