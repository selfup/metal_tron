fn main() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400),
                             Bike::new(800, 400));

    game.bike_one_right();
    game.bike_one_left();
    game.bike_one_up();
    game.bike_one_down();

    game.bike_two_left();
    game.bike_two_right();
    game.bike_two_up();
    game.bike_two_down();

}

/// Game struct. Knows about itself, the grid, and both bikes. Main engine! ***********************

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
        self.bike_one.alive_or_not(&self.grid);
    }

    fn bike_one_left(&mut self) {
        self.bike_one.move_left();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    fn bike_one_up(&mut self) {
        self.bike_one.move_up();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    fn bike_one_down(&mut self) {
        self.bike_one.move_down();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    fn bike_two_right(&mut self) {
        self.bike_two.move_right();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    fn bike_two_left(&mut self) {
        self.bike_two.move_left();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    fn bike_two_up(&mut self) {
        self.bike_two.move_up();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    fn bike_two_down(&mut self) {
        self.bike_two.move_down();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }
}

/// Grid struct. Knows about itself and the two bikes. ********************************************

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

    fn collect_bike_two_trail(&mut self, bike_two: &Bike) {
        self.trails.push((bike_two.x, bike_two.y));
    }
}

/// Bike Struct. Knows about itself and the Grid. *************************************************

struct Bike {
    x: i16,
    y: i16,
    alive: bool,
}

impl Bike {
    fn new(x: i16, y: i16) -> Bike {
        Bike {
            x: x,
            y: y,
            alive: true,
        }
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_up(&mut self) {
        self.y += 1;
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn alive_or_not(&mut self, grid: &Grid) {
        for i in &grid.trails {
            if (self.x, self.y).0 == i.0 && (self.x, self.y).1 == i.1 {
                self.alive = false;
            }
        }
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

// Integration testing ****************************************************************************

#[test]
fn game_can_intialize_with_initlalized_bikes_and_grid() {
    let game = Game::new(Grid::new(800, 0, 800, 0),
                         Bike::new(0, 400),
                         Bike::new(800, 400));

    assert_eq!(game.grid.x_max, 800);
    assert_eq!(game.grid.x_min, 0);
    assert_eq!(game.grid.y_max, 800);
    assert_eq!(game.grid.y_min, 0);
    assert_eq!(game.bike_one.x, 0);
    assert_eq!(game.bike_one.y, 400);
    assert_eq!(game.bike_two.x, 800);
    assert_eq!(game.bike_two.y, 400);
    assert_eq!(game.grid.trails[0], (0, 400));
    assert_eq!(game.grid.trails[1], (800, 400));
    assert_eq!(game.grid.trails.len(), 2);
}

#[test]
fn game_can_take_user_input() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400),
                             Bike::new(800, 400));

    game.bike_one_right();
    assert_eq!(game.bike_one.x, 1);

    game.bike_one_left();
    assert_eq!(game.bike_one.x, 0);

    game.bike_one_up();
    assert_eq!(game.bike_one.y, 401);

    game.bike_one_down();
    assert_eq!(game.bike_one.y, 400);

    game.bike_two_left();
    assert_eq!(game.bike_two.x, 799);

    game.bike_two_right();
    assert_eq!(game.bike_two.x, 800);

    game.bike_two_up();
    assert_eq!(game.bike_two.y, 401);

    game.bike_two_down();
    assert_eq!(game.bike_two.y, 400);
}

#[test]
fn bike_can_detect_other_bikes() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400),
                             Bike::new(800, 400));

    assert_eq!(game.bike_one.alive, true);

    game.bike_one_left();
    assert_eq!(game.bike_one.x, 800);
    assert_eq!(game.bike_one.alive, false);
}

#[test]
fn bike_can_detect_bike_trails() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400),
                             Bike::new(800, 400));

    assert_eq!(game.bike_one.alive, true);

    game.bike_one_right();
    game.bike_one_left();
    assert_eq!(game.bike_one.alive, false);
}