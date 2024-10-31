You got it! Here's the updated code to accept the start date in DD-MM-YYYY format:

use std::io;
use chrono::{NaiveDate, Duration, Weekday, Local};

fn main() {
    // Get the current year automatically
    let year = Local::now().year();

    // Create the holidays based on the current year
    let holidays = get_holidays(year);

    println!("Qual a data da publicação (DD-MM-YYYY):");
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read input.");
    let start_date = NaiveDate::parse_from_str(&start_input.trim(), "%d-%m-%Y").expect("Invalid date format.");

    println!("Qual prazo:");
    let mut days_input = String::new();
    io::stdin().read_line(&mut days_input).expect("Failed to read input.");
    let days: i64 = days_input.trim().parse().expect("Invalid number of days.");

    let mut current_date = start_date;
    let mut count = 0;

    while count < days {
        if !is_weekend(current_date.weekday()) && !is_holiday(current_date, &holidays) {
            count += 1;
        }
        current_date += Duration::days(1);
    }

    let end_date = current_date - Duration::days(1);

    println!("The end date after {} business days is {}.", start_input.trim(), end_date);
}

fn get_holidays(year: i32) -> Vec<NaiveDate> {
    vec![
        NaiveDate::from_ymd(year, 1, 1),   // Ano Novo
        NaiveDate::from_ymd(year, 4, 21),  // Tiradentes
        NaiveDate::from_ymd(year, 5, 1),   // Dia do Trabalho
        NaiveDate::from_ymd(year, 9, 7),   // Independência do Brasil
        NaiveDate::from_ymd(year, 10, 12), // Dia de Nossa Senhora Aparecida
        NaiveDate::from_ymd(year, 11, 2),  // Dia de Finados
        NaiveDate::from_ymd(year, 11, 15), // Proclamação da República
        NaiveDate::from_ymd(year, 12, 25), // Natal
    ]
}

fn is_weekend(weekday: Weekday) -> bool {
    weekday == Weekday::Sat || weekday == Weekday::Sun
}

fn is_holiday(date: NaiveDate, holidays: &Vec<NaiveDate>) -> bool {
    holidays.contains(&date)
}
content_copy
Use code with caution.
Rust

Key Changes:

Prompt format: The println! statement now asks for the start date in the DD-MM-YYYY format:

println!("Enter a start date (DD-MM-YYYY):");
content_copy
Use code with caution.
Rust

parse_from_str format: The NaiveDate::parse_from_str function is now using the format string "%d-%m-%Y" to match the new input format.

With this change, the user can now enter the start date in the desired DD-MM-YYYY format, and the code will correctly parse and process the date.
