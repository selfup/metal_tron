mod game;
mod grid;
mod bike;
mod check;

use game::*;
use grid::*;
use bike::*;

fn main() {
    let mut game = Game::new(Grid::new(800, 0, 800, 0),
                             Bike::new(0, 400),
                             Bike::new(800, 400));

    for _ in 0..1001 {
        game.bike_one_right();
        game.bike_one_left();
        game.bike_one_up();
        game.bike_one_down();

        game.bike_two_left();
        game.bike_two_right();
        game.bike_two_up();
        game.bike_two_down();
    }
}
