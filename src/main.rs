use jiff::{ToSpan, Zoned, civil::Weekday};
use owo_colors::OwoColorize;

fn main() -> Result<(), anyhow::Error> {
    let now = Zoned::now();

    let start = now.first_of_month()?;
    let end = now.last_of_month()?;

    let month_name = month_name(now.month());

    let mut current = start.clone();

    let start_of_week = Weekday::Monday;

    let days_header = (0..=6)
        .map(|d| short_weekday(start_of_week + d))
        .collect::<Vec<_>>()
        .join(" ");

    let monthy_year = format!("{month_name} {year}", year = now.year());

    let width = days_header.len();

    println!("{monthy_year:^width$}");
    println!("{days_header}");
    let offset = start.weekday().to_monday_zero_offset();

    if offset > 0 {
        let empty_space_of_previous_month =
            (0..offset).map(|_| "   ").collect::<Vec<_>>().join(" ");
        print!("{empty_space_of_previous_month} ");
    }

    let (monday_of_week, friday_of_week) = monday_friday(&now);

    loop {
        let day_of_month = current.day();
        let weekday = current.weekday();

        if current == now {
            print!("{:>3} ", day_of_month.blue());
        } else if monday_of_week <= current && current <= friday_of_week {
            print!("{:>3} ", day_of_month.yellow());
        } else {
            print!("{day_of_month:>3} ");
        }
        if weekday == Weekday::Sunday {
            println!();
        }
        current = current.tomorrow()?;
        if current > end {
            break;
        }
    }
    println!();

    Ok(())
}

fn monday_friday(day: &Zoned) -> (Zoned, Zoned) {
    let this_weekday = day.weekday();

    let n = this_weekday.to_monday_zero_offset().days();
    let m = (7 - n.get_days() - 1).days();

    (day - n, day + m)
}

fn month_name(ordinal: i8) -> &'static str {
    match ordinal {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!("the month ordinal should have been between 1 and 12, got: {ordinal}"),
    }
}

fn short_weekday(w: Weekday) -> &'static str {
    match w {
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
        Weekday::Sunday => "Sun",
    }
}
