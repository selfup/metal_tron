use grid::*;
use bike::*;

pub fn all_checks(grid: &mut Grid, bike: &mut Bike) {
    bike.grid_check(grid);
    grid.collect_bike_trail(bike);
    bike.alive_or_not(grid);
}