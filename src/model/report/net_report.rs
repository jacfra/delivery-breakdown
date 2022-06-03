use crate::model::report::delivery_breakdown::DeliveryBreakdown;
use rstring_builder::StringBuilder;
use std::cmp::Ordering;

pub fn generate_net_report(results: &mut Vec<DeliveryBreakdown>) -> StringBuilder {
    let mut builder = StringBuilder::new();

    // make commas obvious
    builder.append("mileage").append(", ");
    builder.append("gross_income").append(", ");
    builder.append("gross_taxes").append(", ");
    builder.append("gross_tax_deductions").append(", ");
    builder.append("net_taxes").append(", ");
    builder.append("net_income").append(", ");
    builder.append("gross_expenses").append(", ");
    builder.append("net_revenue"); // no trailing comma
    builder.append("\n"); // new line

    results.sort_by(|a, b| {
        if a.net_revenue < b.net_revenue {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    for result in results {
        builder.append(result.mileage).append(", ");
        builder.append(result.gross_income).append(", ");
        builder.append(result.gross_taxes).append(", ");
        builder.append(result.gross_tax_deductions).append(", ");
        builder.append(result.net_taxes).append(", ");
        builder.append(result.net_income).append(", ");
        builder.append(result.gross_expenses).append(", ");
        builder.append(result.net_revenue); // no trailing comma
        builder.append("\n"); // newline
    }

    builder
}
