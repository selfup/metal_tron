mod game;
mod grid;
mod bike;

use game::*;
use grid::*;
use bike::*;

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