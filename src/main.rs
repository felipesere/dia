use bpaf::{OptionParser, Parser, construct, short};
use jiff::{
    ToSpan, Zoned,
    civil::{Weekday, date},
    tz::TimeZone,
};
use owo_colors::{OwoColorize, colors::css::Gray};

#[derive(Clone)]
enum QuarterMode {
    Auto,
    Specific(u8),
}

struct Args {
    month: Option<Month>,
    quarter: Option<QuarterMode>,
    year: Option<i16>,
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
                match raw.to_lowercase().as_str() {
                    // Short names (3 letters)
                   "1" | "jan" | "january"  => Ok(Some(Month::Jan)),
                   "2" | "feb" | "february" => Ok(Some(Month::Feb)),
                   "3" | "mar" | "march"    => Ok(Some(Month::Mar)),
                   "4" | "apr" | "april"    => Ok(Some(Month::Apr)),
                   "5" | "may" => Ok(Some(Month::May)),
                   "6" | "jun" | "june"      => Ok(Some(Month::Jun)),
                   "7" | "jul" | "july"      => Ok(Some(Month::Jul)),
                   "8" | "aug" | "august"    => Ok(Some(Month::Aug)),
                   "9" | "sep" | "september" => Ok(Some(Month::Sep)),
                   "10"| "oct" | "october"   => Ok(Some(Month::Oct)),
                   "11"| "nov" | "november"  => Ok(Some(Month::Nov)),
                   "12"| "dez" | "dec" | "december" => Ok(Some(Month::Dez)),
                    _ => Err(anyhow::anyhow!(
                        "Invalid month '{}'. Use month name (jan, january), abbreviation, or number (1-12)",
                        raw
                    )),
                }
            } else {
                Ok(None)
            }

        });

    let quarter_switch = short('q')
        .long("quarter")
        .help("Display the quarter containing the current/specified month")
        .req_flag(QuarterMode::Auto);

    let quarter_num = short('q')
        .long("quarter")
        .argument::<String>("NUM")
        .parse::<_, QuarterMode, anyhow::Error>(|raw| {
            let quarter = raw
                .parse::<u8>()
                .map_err(|_| anyhow::anyhow!("Quarter must be a number between 1 and 4"))?;

            if (1..=4).contains(&quarter) {
                Ok(QuarterMode::Specific(quarter))
            } else {
                Err(anyhow::anyhow!("Quarter must be between 1 and 4"))
            }
        });

    let quarter = construct!([quarter_switch, quarter_num]).optional();

    let year = short('y').long("year").argument::<i16>("YEAR").optional();

    construct!(Args {
        month,
        quarter,
        year
    })
    .to_options()
}

fn main() -> Result<(), anyhow::Error> {
    let args = args().run();

    let requested_year = args.year.unwrap_or_else(|| Zoned::now().year());

    let requested_month = match args.month {
        Some(month) => {
            let now = Zoned::now();
            date(requested_year, month.ordinal(), now.day())
                .to_zoned(TimeZone::system())
                .unwrap()
        }
        None => {
            let today = Zoned::now();
            date(requested_year, today.month(), today.day()).to_zoned(today.time_zone().clone())?
        }
    };

    match args.quarter {
        Some(QuarterMode::Auto) => {
            display_quarter_auto(requested_year, requested_month)?;
        }
        Some(QuarterMode::Specific(quarter_num)) => {
            display_quarter_by_number(requested_year, quarter_num)?;
        }
        None => {
            display_month(requested_month)?;
        }
    }

    Ok(())
}

fn display_month(requested_month: Zoned) -> Result<(), anyhow::Error> {
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
        } else if monday_of_week.date() <= current.date() && current.date() <= friday_of_week.date()
        {
            print!("{:>3} ", day_of_month.yellow());
        } else if matches!(current.weekday(), Weekday::Saturday | Weekday::Sunday) {
            print!("{:>3} ", day_of_month.fg::<Gray>());
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

fn display_quarter_auto(reference_year: i16, reference_month: Zoned) -> Result<(), anyhow::Error> {
    // Determine which quarter the month belongs to
    let month_ordinal = reference_month.month();
    let quarter_num = match month_ordinal {
        1..=3 => 1,   // Q1: Jan, Feb, Mar
        4..=6 => 2,   // Q2: Apr, May, Jun
        7..=9 => 3,   // Q3: Jul, Aug, Sep
        10..=12 => 4, // Q4: Oct, Nov, Dec
        _ => unreachable!(),
    };

    display_quarter_by_number(reference_year, quarter_num)
}

fn display_quarter_by_number(year: i16, quarter_num: u8) -> Result<(), anyhow::Error> {
    let quarter_start_month = match quarter_num {
        1 => 1,  // Q1: Jan, Feb, Mar
        2 => 4,  // Q2: Apr, May, Jun
        3 => 7,  // Q3: Jul, Aug, Sep
        4 => 10, // Q4: Oct, Nov, Dec
        _ => unreachable!("quarter_num should be validated to be 1-4"),
    };

    // Create a date for the first month of the quarter
    let quarter_start = date(year, quarter_start_month, 1)
        .to_zoned(TimeZone::system())
        .unwrap();

    // Display the 3 months of the quarter
    for offset in 0..3 {
        let month = quarter_start.checked_add(offset.months())?;
        display_month(month)?;

        if offset < 2 {
            println!();
        }
    }
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
