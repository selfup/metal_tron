use grid::*;

/// Bike Struct. Knows about itself and Grid. ****************************************************

pub struct Bike {
    pub x: i16,
    pub y: i16,
    pub alive: bool,
}

impl Bike {
    pub fn new(x: i16, y: i16) -> Bike {
        Bike {
            x: x,
            y: y,
            alive: true,
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y += 1;
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn alive_or_not(&mut self, grid: &Grid) {
        for i in &grid.trails {
            if (self.x, self.y).0 == i.0 && (self.x, self.y).1 == i.1 {
                self.alive = false;
            }
        }
    }

    pub fn grid_check(&mut self, grid: &Grid) {
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

#[test]
fn it_initializes_properly() {
    let bike = Bike::new(0, 400);

    assert_eq!(bike.x, 0);
    assert_eq!(bike.y, 400);
}

#[test]
fn it_can_move_around() {
    let mut bike = Bike::new(0, 400);

    bike.move_right();
    assert_eq!(bike.x, 1);

    bike.move_left();
    assert_eq!(bike.x, 0);

    bike.move_up();
    assert_eq!(bike.y, 401);

    bike.move_down();
    assert_eq!(bike.y, 400);
}

#[test]
fn it_can_check_where_it_needs_to_be_on_the_grid() {
    let mut bike = Bike::new(0, 400);
    let grid = Grid::new(800, 0, 800, 0);

    bike.move_left();
    bike.grid_check(&grid);

    assert_eq!(bike.x, 800);
}

#[test]
fn it_knows_if_it_dies() {
    let mut bike = Bike::new(0, 400);
    let grid = Grid::new(800, 0, 800, 0);

    bike.move_left();
    bike.grid_check(&grid);

    assert_eq!(bike.alive, true);
    assert_eq!(bike.x, 800);

    bike.move_right();
    bike.grid_check(&grid);
    bike.alive_or_not(&grid);

    assert_eq!(bike.alive, false);
}
