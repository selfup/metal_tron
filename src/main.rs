fn main() {
    let mut demo = Demo::new();

    demo.print_properties();

    demo.up_to(1001);
}

struct DemoStats {
    init: &'static str,
    modified: &'static str,
    finalized: &'static str,
}

impl DemoStats {
    fn new() -> DemoStats {
        DemoStats {
            init: "initialized",
            modified: "modified",
            finalized: "finalized",
        }
    }
}

struct Demo {
    id: i16,
    status: &'static str,
}

impl Demo {
    fn print_properties(&self) {
        println!("Demo id: {}, Demo status: {}", self.id, self.status);
    }

    fn up_to(&mut self, limit: i16) {
        for i in 1..limit {
            match i {
                1000 => {
                    self.status = DemoStats::new().finalized;
                    self.id = i;
                    self.print_properties();
                }
                _ => {
                    self.status = DemoStats::new().modified;
                    self.id = i;
                    self.print_properties();
                }
            };
        }
    }

    fn new() -> Demo {
        Demo {
            id: 0,
            status: DemoStats::new().init,
        }
    }
}