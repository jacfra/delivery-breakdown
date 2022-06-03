# delivery-breakdown

analytics about making money as a delivery driver

at what point will you have paid for your car after you start using it for deliveries?

![delivery driver revenue projection](/img/delivery-driver-revenue-projection.png)

the heuristic used is fairly simple

given the following input:

```rust
pub struct ExpensePerMile {
    pub gas: f32,               // 0.02 = $0.02 per mile
    pub oil_change: f32,        // 0.02 = $0.02 per mile
    pub windshield_wipers: f32, // 0.02 = $0.02 per mile
    pub battery: f32,           // 0.02 = $0.02 per mile
    pub brake_pads: f32,        // 0.02 = $0.02 per mile
    pub tires: f32,             // 0.02 = $0.02 per mile
    pub other: f32,             // 0.02 = $0.02 per mile
}

pub struct ExpenseLumpSum {
    pub purchase_vehicle: f32, // 20_000.0 == $20,000
    pub accesories: f32,       // 50.0 == $50
}

pub struct TaxConfiguration {
    pub rate: f32,               // 25% == 0.25
    pub write_off_per_mile: f32, // 0.56 = $0.56
}

pub struct VehicleConfiguration {
    pub miles_per_gallon: f32, // 30.0 == 30
}

pub struct WorkConfiguration {
    pub possible_total_mileage: Vec<f32>, // array of different total distances driven
    pub possible_average_pay_per_mile: Vec<f32>, // array of different pay rates earner per mile
    pub price_of_gas_per_gallon: f32, // 3.89 == $3.89
}

```

Which calculates and exposes a delivery breakdown of:

```rust
pub struct DeliveryBreakdown {
    pub mileage: f32,
    pub pay_per_mile: f32,
    pub gross_income: f32,
    pub gross_taxes: f32,
    pub gross_tax_deductions: f32,
    pub net_taxes: f32,
    pub net_income: f32,
    pub gross_expenses: f32,
    pub net_revenue: f32,
}
```

and then writes a csv sorted by net revenue

todo:

write something that makes it easy to update the actual miles driven and earnings generated
