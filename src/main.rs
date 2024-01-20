slint::include_modules!();
use slint::SharedString;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_calculate_savings({
        let ui_handle = ui.as_weak();
        
        move |savings: SharedString, interest: SharedString, deposits: SharedString, years: SharedString, payout_years: SharedString| {
            let ui = ui_handle.unwrap();
            let float_savings: f64 = parse_string_to_float(savings);
            let mut float_interest: f64 = parse_string_to_float(interest);
            let float_deposits: f64 = parse_string_to_float(deposits);
            let float_years: f64 = parse_string_to_float(years);
            if float_interest > 1.0 {
                float_interest = float_interest / 100.0;
            };
            let total_savings: String = calculate_interest(float_savings, float_interest, float_deposits, float_years);
            let float_total_savings: f64 = parse_string_to_float(slint::SharedString::from(total_savings.clone()));
            let float_payout_years: f64 = parse_string_to_float(payout_years);
            let payout_months: f64 = float_payout_years * 12.0 * -1.0;
            let retirement_income: String = calculate_retirement_income(float_total_savings, payout_months, float_interest);
            ui.set_total_savings(total_savings.clone().into());
            ui.set_total_savings_adjusted(inflation_adjusted(total_savings.clone(), float_payout_years).into());
            ui.set_monthly_salary(retirement_income.clone().into());
            ui.set_monthly_salary_adjusted(inflation_adjusted(retirement_income.clone(), float_payout_years).into());
        }
    });

    ui.run()
}

fn calculate_interest(savings: f64, interest: f64, deposits: f64, years: f64) -> String {
    let months: i64 = (years as i64) * 12;
    let monthly_interest: f64 = 1.0 + (interest / 12.0);
    let mut final_balance :f64 = savings;
    for _ in 0..months {
        final_balance *= monthly_interest;
        final_balance += deposits;
    };
    return format!("{:.2}", final_balance);
}

fn calculate_retirement_income(final_balance: f64, payout_months: f64, interest: f64) -> String {
    let monthly_income: f64 = final_balance / ((1.0 - (1.0 + interest / 12.0).powf(payout_months)) / (interest / 12.0));
    return format!("{:.2}", monthly_income);
}

fn parse_string_to_float(input: SharedString) -> f64 {
    if input == "" {
        return 0.0
    };
    return input.trim().parse::<f64>().unwrap();
}

fn inflation_adjusted(dollars: String, years: f64) -> String {
    let adjusted: f64 = parse_string_to_float(slint::SharedString::from(dollars.clone())) * (0.97_f64).powf(years);
    return format!("{:.2}", adjusted)
}