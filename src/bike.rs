use grid::*;

/// Bike Struct. Knows about itself and the Grid. *************************************************

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