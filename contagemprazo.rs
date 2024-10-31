use std::io;
use chrono::{NaiveDate, Duration, Weekday, Local, Datelike};

fn main() {
    let year = Local::now().year();

    let holidays = get_holidays(year);

    println!("Qual a data da publicação (DD-MM-YYYY):");
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).unwrap();
    let start_date = NaiveDate::parse_from_str(&start_input.trim(), "%d-%m-%Y")
        .unwrap();

    println!("Qual prazo:");
    let mut days_input = String::new();
    io::stdin().read_line(&mut days_input).unwrap();
    let days: i64 = days_input.trim().parse().unwrap();

    let mut current_date = start_date;
    let mut count = 0;

    while count < days {
        current_date += Duration::days(1);
        if !is_weekend(current_date.weekday()) && !is_holiday(current_date, &holidays) {
            count += 1;
        }
    }

    println!("Data final após {} dias úteis: {}", days, current_date.format("%d-%m-%Y"));
}

fn get_holidays(year: i32) -> Vec<NaiveDate> {
    vec![
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),   // Ano Novo
        NaiveDate::from_ymd_opt(year, 4, 21).unwrap(),  // Tiradentes
        NaiveDate::from_ymd_opt(year, 5, 1).unwrap(),   // Dia do Trabalho
        NaiveDate::from_ymd_opt(year, 9, 7).unwrap(),   // Independência do Brasil
        NaiveDate::from_ymd_opt(year, 10, 12).unwrap(), // Dia de Nossa Senhora Aparecida
        NaiveDate::from_ymd_opt(year, 11, 2).unwrap(),  // Dia de Finados
        NaiveDate::from_ymd_opt(year, 11, 15).unwrap(), // Proclamação da República
        NaiveDate::from_ymd_opt(year, 12, 25).unwrap(), // Natal
    ]
}

fn is_weekend(weekday: Weekday) -> bool {
    weekday == Weekday::Sat || weekday == Weekday::Sun
}

fn is_holiday(date: NaiveDate, holidays: &[NaiveDate]) -> bool {
    holidays.contains(&date)
}
