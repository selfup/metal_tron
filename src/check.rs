use grid::*;
use bike::*;

pub fn all_checks(mut grid: &mut Grid, mut bike: &mut Bike) {
    bike.grid_check(grid);
    grid.collect_bike_trail(bike);
    bike.alive_or_not(grid);
}
