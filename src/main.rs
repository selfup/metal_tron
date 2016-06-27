fn main() {
    let mut bike_one = Bike::init();

    bike_one.print_properties();

    bike_one.move_right();

    bike_one.print_properties();
}

struct Grid {
    
}

struct Bike {
    x: i32,
    y: i32,
    status: &'static str,
}

impl Bike {
    fn init() -> Bike {
        Bike {
            x: 0,
            y: 0,
            status: "alive",
        }
    }

    fn print_properties(&self) {
        println!("Bike coords: x -> {}, y -> {}, Bike status: {}",
                 self.x,
                 self.y,
                 self.status);
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn status_check(&mut self, grid: Grid) {

    }
}
