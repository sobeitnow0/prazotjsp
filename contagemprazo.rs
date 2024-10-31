use std::io;
use chrono::{NaiveDate, Duration, Weekday, Local, Datelike};

fn main() {
    let year = Local::now().year();
    let holidays = get_holidays(year);

    println!("Qual a data da publicação (DD-MM-YYYY):");
    let start_date = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match NaiveDate::parse_from_str(input.trim(), "%d-%m-%Y") {
            Ok(date) => break date,
            Err(_) => println!("Data inválida. Use o formato DD-MM-YYYY:"),
        }
    };

    println!("Qual prazo:");
    let days: i64 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Número inválido. Digite novamente:"),
        }
    };

    let mut current_date = start_date + Duration::days(1);
    let mut count = 0;
    let mut all_days = Vec::new();

    while is_weekend(current_date.weekday()) || is_holiday(current_date, &holidays) {
        all_days.push((current_date, false));
        current_date += Duration::days(1);
    }

    while count < days {
        let is_business_day = !is_weekend(current_date.weekday()) && !is_holiday(current_date, &holidays);
        all_days.push((current_date, is_business_day));
        
        if is_business_day {
            count += 1;
        }
        current_date += Duration::days(1);
    }

    println!("\nContagem\tData");
    println!("-----------------------------------------");

    let weekdays = ["Domingo", "Segunda", "Terça", "Quarta", "Quinta", "Sexta", "Sábado"];
    let mut business_day_count = 0;

    for (date, is_business) in all_days {
        let weekday = weekdays[date.weekday().num_days_from_sunday() as usize];
        let day_info = if is_business {
            business_day_count += 1;
            format!("{}\t\t{} - {}", 
                business_day_count,
                date.format("%d/%m/%Y"),
                weekday)
        } else {
            let reason = if is_holiday(date, &holidays) {
                "Feriado"
            } else {
                "Final de Semana"
            };
            format!("X\t\t{} - {} ({})", 
                date.format("%d/%m/%Y"),
                weekday,
                reason)
        };
        println!("{}", day_info);
    }
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
