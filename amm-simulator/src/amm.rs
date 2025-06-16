#[derive(Debug)]
pub struct AMMPool {
    pub x: f64,
    pub y: f64,
    pub fee: f64,
}

impl AMMPool {
    pub fn new(x: f64, y: f64, fee: f64) -> Self {
        Self { x, y, fee }
    }

    pub fn get_k(&self) -> f64 {
        self.x * self.y
    }

    pub fn swap_x_for_y(&mut self, x_in: f64) -> f64 {
        let x_in_with_fee = x_in * (1.0 - self.fee);
        let new_x = self.x + x_in_with_fee;
        let new_y = self.get_k() / new_x;
        let y_out = self.y - new_y;

        self.x += x_in_with_fee;
        self.y = new_y;

        y_out
    }

    pub fn swap_y_for_x(&mut self, y_in: f64) -> f64 {
        let y_in_with_fee = y_in * (1.0 - self.fee);
        let new_y = self.y + y_in_with_fee;
        let new_x = self.get_k() / new_y;
        let x_out = self.x - new_x;

        self.y += y_in_with_fee;
        self.x = new_x;

        x_out
    }

    pub fn add_liquidity(&mut self, x: f64) -> f64 {
        let y_required = x * self.y / self.x;
        self.x += x;
        self.y += y_required;
        y_required
    }

    pub fn status(&self) -> (f64, f64, f64) {
        println!("\n Pool Status \n x: {:.4}, y: {:.4}, k: {:.4}", self.x, self.y, self.get_k());
        (self.x, self.y, self.get_k())
    }
}


