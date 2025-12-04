use bpaf::{OptionParser, Parser, construct, short};
use jiff::{
    ToSpan, Zoned,
    civil::{Weekday, date},
    tz::TimeZone,
};
use owo_colors::OwoColorize;

struct Args {
    month: Option<Month>,
}

enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dez,
}
impl Month {
    fn ordinal(&self) -> i8 {
        match &self {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dez => 12,
        }
    }
}

fn args() -> OptionParser<Args> {
    let month = short('m')
        .long("month")
        .help("Month to display")
        .argument::<String>("MONTH")
        .optional()
        .parse::<_, Option<Month>, anyhow::Error>(|raw: Option<String>| {
            if let Some(raw) = raw {
                return match raw.to_lowercase().as_str() {
                    // Short names (3 letters)
                    "jan" => Ok(Some(Month::Jan)),
                    "feb" => Ok(Some(Month::Feb)),
                    "mar" => Ok(Some(Month::Mar)),
                    "apr" => Ok(Some(Month::Apr)),
                    "may" => Ok(Some(Month::May)),
                    "jun" => Ok(Some(Month::Jun)),
                    "jul" => Ok(Some(Month::Jul)),
                    "aug" => Ok(Some(Month::Aug)),
                    "sep" => Ok(Some(Month::Sep)),
                    "oct" => Ok(Some(Month::Oct)),
                    "nov" => Ok(Some(Month::Nov)),
                    "dez" | "dec" => Ok(Some(Month::Dez)),

                    // Full month names
                    "january" => Ok(Some(Month::Jan)),
                    "february" => Ok(Some(Month::Feb)),
                    "march" => Ok(Some(Month::Mar)),
                    "april" => Ok(Some(Month::Apr)),
                    "june" => Ok(Some(Month::Jun)),
                    "july" => Ok(Some(Month::Jul)),
                    "august" => Ok(Some(Month::Aug)),
                    "september" => Ok(Some(Month::Sep)),
                    "october" => Ok(Some(Month::Oct)),
                    "november" => Ok(Some(Month::Nov)),
                    "december" => Ok(Some(Month::Dez)),

                    // Month numbers
                    "1" => Ok(Some(Month::Jan)),
                    "2" => Ok(Some(Month::Feb)),
                    "3" => Ok(Some(Month::Mar)),
                    "4" => Ok(Some(Month::Apr)),
                    "5" => Ok(Some(Month::May)),
                    "6" => Ok(Some(Month::Jun)),
                    "7" => Ok(Some(Month::Jul)),
                    "8" => Ok(Some(Month::Aug)),
                    "9" => Ok(Some(Month::Sep)),
                    "10" => Ok(Some(Month::Oct)),
                    "11" => Ok(Some(Month::Nov)),
                    "12" => Ok(Some(Month::Dez)),

                    _ => Err(anyhow::anyhow!(
                        "Invalid month '{}'. Use month name (jan, january), abbreviation, or number (1-12)",
                        raw
                    )),
                };
            }

            Ok(None)
        });

    construct!(Args { month }).to_options()
}

fn main() -> Result<(), anyhow::Error> {
    let args = args().run();

    let requested_month = match args.month {
        Some(month) => {
            let now = Zoned::now();
            date(now.year(), month.ordinal(), now.day())
                .to_zoned(TimeZone::system())
                .unwrap()
        }
        None => Zoned::now(),
    };
    let today = Zoned::now();

    let start = requested_month.first_of_month()?;
    let end = requested_month.last_of_month()?;

    let month_name = month_name(requested_month.month());

    let mut current = start.clone();

    let start_of_week = Weekday::Monday;

    let days_header = (0..=6)
        .map(|d| short_weekday(start_of_week + d))
        .collect::<Vec<_>>()
        .join(" ");

    let monthy_year = format!("{month_name} {year}", year = requested_month.year());

    let width = days_header.len();

    println!("{monthy_year:^width$}");
    println!("{days_header}");
    let offset = start.weekday().to_monday_zero_offset();

    if offset > 0 {
        let empty_space_of_previous_month =
            (0..offset).map(|_| "   ").collect::<Vec<_>>().join(" ");
        print!("{empty_space_of_previous_month} ");
    }

    let (monday_of_week, friday_of_week) = monday_friday(&today);

    loop {
        let day_of_month = current.day();
        let weekday = current.weekday();

        if current.date() == today.date() {
            print!("{:>3} ", day_of_month.blue());
        } else if monday_of_week.date() <= current.date() && current.date() <= friday_of_week.date() {
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
