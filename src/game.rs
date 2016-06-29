use grid::*;
use bike::*;

/// Game struct. Knows about itself, the grid, and both bikes. Main engine! ***********************

pub struct Game {
    pub grid: Grid,
    pub bike_one: Bike,
    pub bike_two: Bike,
}

impl Game {
    pub fn new(grid: Grid, bike_one: Bike, bike_two: Bike) -> Game {
        Game {
            grid: grid,
            bike_one: bike_one,
            bike_two: bike_two,
        }
    }

    pub fn bike_one_right(&mut self) {
        self.bike_one.move_right();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_one_left(&mut self) {
        self.bike_one.move_left();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_one_up(&mut self) {
        self.bike_one.move_up();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_one_down(&mut self) {
        self.bike_one.move_down();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_two_right(&mut self) {
        self.bike_two.move_right();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_two_left(&mut self) {
        self.bike_two.move_left();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_two_up(&mut self) {
        self.bike_two.move_up();
        self.all_checks(self.grid, self.bike_two);
    }

    pub fn bike_two_down(&mut self) {
        self.bike_two.move_down();
        self.all_checks(self.grid, self.bike_two);
    }

    fn all_checks(&self, mut grid: Grid, mut bike: Bike) {
        bike.grid_check(&grid);
        grid.collect_bike_one_trail(&bike);
        bike.alive_or_not(&grid);
    }
}

#[test]
fn game_can_intialize_with_initlalized_bikes_and_a_grid() {
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
fn it_can_detect_other_bikes_and_figure_out_if_it_is_alive_or_not() {
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