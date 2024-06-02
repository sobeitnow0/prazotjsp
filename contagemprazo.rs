use std::io;
use chrono::{NaiveDate, Duration, Weekday};

fn main() {
    // Defina aqui sua lista de feriados nacionais do Brasil
    let holidays = vec![
        NaiveDate::from_ymd(2022, 1, 1),   // Ano Novo
        NaiveDate::from_ymd(2022, 4, 21),  // Tiradentes
        NaiveDate::from_ymd(2022, 5, 1),   // Dia do Trabalho
        NaiveDate::from_ymd(2022, 9, 7),   // Independência do Brasil
        NaiveDate::from_ymd(2022, 10, 12), // Dia de Nossa Senhora Aparecida
        NaiveDate::from_ymd(2022, 11, 1),  // Dia de Todos os Santos
        NaiveDate::from_ymd(2022, 12, 25), // Natal
    ];

    println!("Enter a start date (YYYY-MM-DD):");
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read input.");
    let start_date = NaiveDate::parse_from_str(&start_input.trim(), "%Y-%m-%d").expect("Invalid date format.");

    println!("Enter the number of days:");
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

fn is_weekend(weekday: Weekday) -> bool {
    weekday == Weekday::Sat || weekday == Weekday::Sun
}

fn is_holiday(date: NaiveDate, holidays: &Vec<NaiveDate>) -> bool {
    holidays.contains(&date)
}
