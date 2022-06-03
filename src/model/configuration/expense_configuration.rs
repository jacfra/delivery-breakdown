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
    pub oil_change: f32,
    pub windshield_wipers: f32,
    pub battery: f32,
    pub brake_pads: f32,
    pub tires: f32,
    pub other: f32,
}

impl ExpensePerMile {
    pub fn total(&self) -> f32 {
        self.gas
            + self.oil_change
            + self.windshield_wipers
            + self.battery
            + self.brake_pads
            + self.tires
            + self.other
    }
}
