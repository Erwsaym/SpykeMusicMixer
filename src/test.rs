pub struct Test {
    a: i32
}

impl Test {
    pub fn new() -> Test {
        Test {
            a: 5
        }
    }

    pub fn set_a(&mut self, v : i32) {
        self.a = v;
    }

    pub fn get_a(& self) -> & i32 {
        & self.a
    }
}