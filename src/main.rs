use chrono::Local;
use chrono::{Datelike, NaiveDate};
mod utils;
use utils::config::{NEPALI_MONTH_NAMES, NEPALI_YEARS};

fn main() {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let devanagari_time = time_str
        .chars()
        .map(to_devanagari_numeral)
        .collect::<String>();

    println!("{}", to_devanagari(get_nepali_date(2023, 4, 14))); // Start of 2080 BS
    println!("{}", to_devanagari(get_nepali_date(2024, 7, 31))); // Random date
    println!("{}", to_devanagari(get_nepali_date(2043, 4, 13))); // End of 2099 BS
    println!("समय: {}", devanagari_time);
}

fn get_nepali_date(year: i32, month: u32, day: u32) -> String {
    let ref_nepali_date = NaiveDate::from_ymd_opt(2080, 1, 1).unwrap();
    let ref_gregorian_date = NaiveDate::from_ymd_opt(2023, 4, 14).unwrap();
    let input_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    let day_difference = input_date
        .signed_duration_since(ref_gregorian_date)
        .num_days();

    let mut nepali_year = ref_nepali_date.year();
    let mut nepali_month = 1;
    let mut nepali_day = 1;
    let mut remaining_days = day_difference;

    while remaining_days != 0 {
        let year_index = (nepali_year - 2080) as usize;
        if year_index >= NEPALI_YEARS.len() {
            return format!("Date out of supported range");
        }

        let (current_year_months, year_days) = &NEPALI_YEARS[year_index];

        if remaining_days > 0 {
            if remaining_days > *year_days as i64 {
                remaining_days -= *year_days as i64;
                nepali_year += 1;
            } else {
                for (month, &days) in current_year_months.iter().enumerate() {
                    if remaining_days > days as i64 {
                        remaining_days -= days as i64;
                        nepali_month += 1;
                    } else {
                        nepali_day += remaining_days as usize;
                        remaining_days = 0;
                        break;
                    }
                }
            }
        } else {
            if -remaining_days >= *year_days as i64 {
                remaining_days += *year_days as i64;
                nepali_year -= 1;
            } else {
                for (month, &days) in current_year_months.iter().enumerate().rev() {
                    if -remaining_days >= days as i64 {
                        remaining_days += days as i64;
                        nepali_month -= 1;
                        if nepali_month == 0 {
                            nepali_month = 12;
                            nepali_year -= 1;
                        }
                    } else {
                        nepali_day = (days as i64 + remaining_days + 1) as usize;
                        remaining_days = 0;
                        break;
                    }
                }
            }
        }
    }

    format!(
        "{} {} {}",
        nepali_day,
        NEPALI_MONTH_NAMES[nepali_month - 1],
        nepali_year
    )
}
fn to_devanagari_numeral(c: char) -> char {
    match c {
        '0' => '०',
        '1' => '१',
        '2' => '२',
        '3' => '३',
        '4' => '४',
        '5' => '५',
        '6' => '६',
        '7' => '७',
        '8' => '८',
        '9' => '९',
        ':' => ':',
        _ => c,
    }
}

fn to_devanagari(input: String) -> String {
    input
        .chars()
        .map(|c| match c {
            '0'..='9' => to_devanagari_numeral(c),
            _ => c,
        })
        .collect()
}
