use crate::ExpenseLumpSum;
use crate::ExpensePerMile;
use crate::TaxConfiguration;
use crate::WorkConfiguration;

pub fn calculate_delivery_driver_breakdown(
    work_configration: WorkConfiguration,
    tax_configuration: TaxConfiguration,
    expense_per_mile: ExpensePerMile,
    expense_lump_sum: ExpenseLumpSum,
) -> Vec<DeliveryBreakdown> {
    let mut results: Vec<DeliveryBreakdown>;

    results = Vec::new();

    for mileage in &work_configration.possible_total_mileage {
        for pay_per_mile in &work_configration.possible_average_pay_per_mile {
            let gross_income = mileage * pay_per_mile;
            let gross_taxes = gross_income * tax_configuration.rate;
            let gross_tax_deductions = mileage * tax_configuration.write_off_per_mile;
            let mut net_taxes = gross_taxes - gross_tax_deductions;
            if net_taxes < 0.0 {
                net_taxes = 0.0;
            }

            let net_income = gross_income - net_taxes;
            let gross_expenses = (expense_per_mile.total() * mileage) + expense_lump_sum.total();
            let net_revenue = net_income - gross_expenses;

            results.push(DeliveryBreakdown {
                mileage: *mileage,
                pay_per_mile: *pay_per_mile,
                net_revenue: net_revenue,
                gross_income: gross_income,
                gross_taxes: gross_taxes,
                net_taxes: net_taxes,
                net_income: net_income,
                gross_expenses: gross_expenses,
                gross_tax_deductions: gross_tax_deductions,
            })
        }
    }
    results
}

#[derive(Clone)]
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
