mod model;
mod utility;

use crate::model::configuration::expense_configuration::ExpenseLumpSum;
use crate::model::configuration::expense_configuration::ExpensePerMile;
use crate::model::configuration::tax_configuration::TaxConfiguration;
use crate::model::configuration::vehicle_configuration::VehicleConfiguration;
use crate::model::configuration::work_configuration::WorkConfiguration;
use crate::model::report::delivery_breakdown::calculate_delivery_driver_breakdown;
use crate::model::report::net_report;
use crate::utility::util::{create_directory, write_file};
use std::path::Path;

fn main() {
    let work_configration = WorkConfiguration {
        possible_total_mileage: vec![
            0.0, 5_000.0, 10_000.0, 15_000.0, 20_000.0, 25_000.0, 30_000.0, 35_000.0, 40_000.0,
        ],
        possible_average_pay_per_mile: vec![1.00, 1.25, 1.50, 1.75, 2.00, 2.25, 2.50],
        price_of_gas_per_gallon: 3.89,
    };

    let expense_lump_sum = ExpenseLumpSum {
        purchase_vehicle: 17_000.00,
        accesories: 50.00,
    };

    let tax_configuration = TaxConfiguration {
        rate: 0.25,
        write_off_per_mile: 0.56, //2021
    };

    let vehicle_configuration = VehicleConfiguration {
        miles_per_gallon: 30.0,
    };

    // maintenance
    // ---------
    // oil_change - 0.01/mi - every 5,000 miles at $50
    // windshield_wipers - 0.01/mi -  5,000 miles at $50
    // battery - 0.01/mi - 10,000 miles at $100
    // brake_pads - 0.02/mi - 30,000 miles at $400
    // tires - 0.02/mi - 60,000 miles at $1000

    let expense_per_mile = ExpensePerMile {
        gas: work_configration.price_of_gas_per_gallon / vehicle_configuration.miles_per_gallon, // $0.12
        maintenance: 0.07,
        other: 0.02, // 10,000 miles at $200
    };

    let results = calculate_delivery_driver_breakdown(
        work_configration,
        tax_configuration,
        expense_per_mile,
        expense_lump_sum,
    );

    let report_dir = Path::new("./report");
    create_directory(report_dir.to_path_buf());

    let net_report_path = report_dir.join("net_report.csv");
    let net_report_data = net_report::generate_net_report(&mut results.clone());
    write_file(net_report_path, net_report_data);
}
