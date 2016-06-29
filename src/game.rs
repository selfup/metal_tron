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
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    pub fn bike_one_left(&mut self) {
        self.bike_one.move_left();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    pub fn bike_one_up(&mut self) {
        self.bike_one.move_up();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    pub fn bike_one_down(&mut self) {
        self.bike_one.move_down();
        self.bike_one.grid_check(&self.grid);
        self.grid.collect_bike_one_trail(&self.bike_one);
        self.bike_one.alive_or_not(&self.grid);
    }

    pub fn bike_two_right(&mut self) {
        self.bike_two.move_right();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    pub fn bike_two_left(&mut self) {
        self.bike_two.move_left();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    pub fn bike_two_up(&mut self) {
        self.bike_two.move_up();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }

    pub fn bike_two_down(&mut self) {
        self.bike_two.move_down();
        self.bike_two.grid_check(&self.grid);
        self.grid.collect_bike_two_trail(&self.bike_two);
        self.bike_two.alive_or_not(&self.grid);
    }
}