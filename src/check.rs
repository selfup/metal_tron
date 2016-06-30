use grid::*;
use bike::*;

/// Check mod. Checks Grid, Collects Bike trails, and checks if Bike.alive == true. **************

pub fn all_checks(grid: &mut Grid, bike: &mut Bike) {
    bike.grid_check(grid);
    grid.collect_bike_trail(bike);
    bike.alive_or_not(grid);
}
