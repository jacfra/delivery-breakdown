use std::cmp::Ordering;

fn main() {
    let possible_mileage = [
        5_000.0, 10_000.0, 15_000.0, 20_000.0, 25_000.0, 30_000.0, 35_000.0, 40_000.0,
    ];
    let possible_pay_per_mile = [1.00, 1.25, 1.50, 1.75, 2.00, 2.25, 2.50, 2.75, 3.00];

    let expense_to_purchase_vehicle = 17_000.00;
    let expense_accesories = 50.00;

    let total_expense_lump_sum = expense_to_purchase_vehicle + expense_accesories;

    let expense_gas_per_mile = 0.02;
    let expense_oil_per_mile = 0.01;
    let expense_wipers_per_mile = 0.01;
    let expense_battery_per_mile = 0.01;
    let expense_brake_pads_per_mile = 0.02;
    let expense_tires_per_mile = 0.02;

    let total_expense_per_mile = expense_gas_per_mile
        + expense_oil_per_mile
        + expense_wipers_per_mile
        + expense_battery_per_mile
        + expense_brake_pads_per_mile
        + expense_tires_per_mile;

    let tax_rate = 0.25;
    let tax_write_off_per_mile = 0.56;

    let mut results: Vec<DeliveryBreakdown>;

    results = Vec::new();

    for mileage in &possible_mileage {
        for pay_per_mile in &possible_pay_per_mile {
            let gross_income = mileage * pay_per_mile;

            let gross_taxes = gross_income * tax_rate;

            let mut net_taxes = gross_taxes - (mileage * tax_write_off_per_mile);

            if net_taxes < 0.0 {
                net_taxes = 0.0;
            }

            let net_income = gross_income - net_taxes;

            let gross_expenses = (total_expense_per_mile * mileage) + total_expense_lump_sum;

            let net = net_income - gross_expenses;

            results.push(DeliveryBreakdown {
                mileage: *mileage,
                pay_per_mile: *pay_per_mile,
                net: net,
                gross_income: gross_income,
                gross_taxes: gross_taxes,
                net_taxes: net_taxes,
                net_income: net_income,
                gross_expenses: gross_expenses,
            })
        }
    }

    pay_per_mile_report(&mut results);
    mileage_report(&mut results);
}

fn pay_per_mile_report(results: &mut Vec<DeliveryBreakdown>) {
    println!("pay_per_mile, mileage, gross_income, gross_taxes, net_taxes, net_income, gross_expenses, net");

    results.sort_by(|a, b| {
        if a.pay_per_mile < b.pay_per_mile {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    for result in results {
        println!(
            "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            result.pay_per_mile,
            result.mileage,
            result.gross_income,
            result.gross_taxes,
            result.net_taxes,
            result.net_income,
            result.gross_expenses,
            result.net
        );
    }
}

fn mileage_report(results: &mut Vec<DeliveryBreakdown>) {
    println!("mileage, pay_per_mile, gross_income, gross_taxes, net_taxes, net_income, gross_expenses, net");

    results.sort_by(|a, b| {
        if a.mileage < b.mileage {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    for result in results {
        println!(
            "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            result.mileage,
            result.pay_per_mile,
            result.gross_income,
            result.gross_taxes,
            result.net_taxes,
            result.net_income,
            result.gross_expenses,
            result.net
        );
    }
}

pub struct DeliveryBreakdown {
    mileage: f32,
    pay_per_mile: f32,
    gross_income: f32,
    gross_taxes: f32,
    net_taxes: f32,
    net_income: f32,
    gross_expenses: f32,
    net: f32,
}
