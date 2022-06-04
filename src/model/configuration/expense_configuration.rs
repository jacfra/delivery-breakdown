pub struct ExpenseLumpSum {
    pub purchase_vehicle: f32,
    pub accesories: f32,
}

impl ExpenseLumpSum {
    pub fn total(&self) -> f32 {
        self.accesories + self.purchase_vehicle
    }
}

pub struct ExpensePerMile {
    pub gas: f32,
    pub maintenance: f32,
    pub other: f32,
}

impl ExpensePerMile {
    pub fn total(&self) -> f32 {
        self.gas + self.maintenance + self.other
    }
}
