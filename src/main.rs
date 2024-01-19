slint::include_modules!();
use slint::SharedString;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_calculate_savings({
        let ui_handle = ui.as_weak();
        
        move |savings: SharedString, interest: SharedString, deposits: SharedString, years: SharedString| {
            let ui = ui_handle.unwrap();
            let float_savings:f64 = parse_string_to_float(savings);
            let mut float_interest:f64 = parse_string_to_float(interest);
            let float_deposits:f64 = parse_string_to_float(deposits);
            let float_years:f64 = parse_string_to_float(years);
            if float_interest > 1.0 {
                float_interest = float_interest / 100.0;
            };
            ui.set_total_savings(calculate_interest(float_savings, float_interest, float_deposits, float_years).into());
        }
    });

    ui.on_calculate_monthly_salary({
        let ui_handle = ui.as_weak();
        
        move |final_balance: SharedString, payout_years: SharedString, interest: SharedString| {
            println!("{}", final_balance);
            println!("{}", payout_years);
            let ui = ui_handle.unwrap();
            let float_final_balance:f64 = parse_string_to_float(final_balance);
            let float_payout_years:f64 = parse_string_to_float(payout_years);
            let payout_months = float_payout_years * 12.0 * -1.0;
            let mut float_interest:f64 = parse_string_to_float(interest);
            if float_interest > 1.0 {
                float_interest = float_interest / 100.0;
                };
            ui.set_monthly_salary(calculate_retirement_income(float_final_balance, payout_months, float_interest).into());
        }
    });

    ui.run()
}

fn calculate_interest(savings:f64, interest:f64, deposits:f64, years:f64) -> String {
    let months: i64 = (years as i64) * 12;
    let monthly_interest: f64 = 1.0 + (interest / 12.0);
    let mut final_balance :f64 = savings;
    for _ in 0..months {
        final_balance *= monthly_interest;
        final_balance += deposits;
    }
    return format!("{:.2}", final_balance);
}

fn calculate_retirement_income(final_balance:f64, payout_months:f64, interest:f64) -> String {
    let monthly_income: f64 = final_balance / ((1.0 - (1.0 + interest / 12.0).powf(payout_months)) / (interest / 12.0));
    return format!("{:.2}", monthly_income);
}

fn parse_string_to_float(input: SharedString) -> f64 {
    if input == "" {
        return 0.0
    }
    return input.trim().parse::<f64>().unwrap();
}