//! pentazygonic calendar - the Five Great Months of the Pentazygon
// UMODK... 0(::9), 1(::8), 2(::7), 3(::6), 4(::5)
// MOK(DU) 1, 2, 3, 8, 7, 5 (0, 3, 6, 9)
// psz: 5 * 72(=9) = 365
use std::time::{SystemTime, UNIX_EPOCH};
#[macro_use] extern crate prettytable;
use prettytable::{format, Cell, Row, Table, Attr, color};
use std::convert::TryInto;

const DAYS_IN_YEAR: u32 = 365;
const MONTHS_IN_YEAR: u32 = 5;
const FIVE_GREAT_MONTHS: u32 = 5;
const DAYS_IN_MONTH: u32 = 73;
const SECONDS_IN_YEAR: u64 = 60 * 60 * 24 * 73 * 5;
const SECONDS_IN_MONTH: u64 = SECONDS_IN_YEAR / 5;
const SECONDS_IN_DAY: u64 = 60 * 60 * 24;
const INCIPIT_AOK: u64 = 2000;

struct Curr {
    day: u64,
    month: u64,
    year: u64,
}

impl Curr {
    fn new() -> Self {
        Curr{day: 0, month: 0, year: 0}
    }
}

fn main() {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap();

    let mut curr = Curr::new();

    let years_since = since_the_epoch.as_secs() / SECONDS_IN_YEAR;
    curr.year = 1970 + years_since;
    let leaps_since = years_since / 4; // cursed gregorian intercals... implicit floor
    let months_since = since_the_epoch.as_secs() / SECONDS_IN_MONTH;
    curr.month = months_since % FIVE_GREAT_MONTHS as u64;
    let days_since = since_the_epoch.as_secs() / SECONDS_IN_DAY;
    let day_number = days_since % 365 - leaps_since; // days since beginning of year
    curr.day = day_number % DAYS_IN_MONTH as u64;
    println!("D/M/Y: {:0>2} / {:0>2} / {}", curr.day, curr.month, curr.year - INCIPIT_AOK);

    date_string(&curr);
    date_table(&curr);
}

fn date_string(curr: &Curr) {
    print!("The {}", curr.day);
    if curr.day % 10 == 1 {
        print!("st");
    } else if curr.day % 10 == 2 {
        print!("nd");
    } else if curr.day % 10 == 3 {
        print!("rd");
    } else {
        print!("th");
    }
    print!(" day of ");

    match curr.month {
        1 => print!("Uttunul"),
        2 => print!("Murrumur"),
        3 => print!("Oddubb"),
        4 => print!("Djynxx"),
        5 => print!("Khattak"),
        _ => {}
    }

    print!(" in the year ");
    print!("{}", curr.year - INCIPIT_AOK);
    print!(" AOK.\n\n")
}

fn date_table(curr: &Curr) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row!["L", "Du", "Do", "Ix", "Ig", "Id", "K", "Sg", "Sd"]);
    // 00-Lurgo, 01-Duoddod, 02-Doogu, 03-Ixix, 04-Ixigool, 05-Ixidod, 06-Krako, 07-Sukugool
    // 08-Skoodu, 09-Skarkix

    let mut title_str = match curr.month {
        1 => "Uttunul",
        2 => "Murrumur",
        3 => "Oddubb",
        4 => "Djynxx",
        5 => "Khattak",
        _ => ""
    }
    .to_string();
    title_str += &(" ".to_string() + &(curr.year - INCIPIT_AOK).to_string());
    println!("{:^34}", title_str);

    let mut week = Vec::<String>::new();
    for i in 1..=DAYS_IN_MONTH {
        week.push(i.to_string());
        if i % 9 == 0 && i != 0 {
            push_week_to_table(&week, curr.day, &mut table);
            week.clear();
        }
    }
    if !week.is_empty() {
        push_week_to_table(&week, curr.day, &mut table);
    }
    table.printstd();
}

fn push_week_to_table(week: &Vec<String>, curr_day: u64, table: &mut Table) {
    let mut cells = Vec::<Cell>::new();
    for s in week {
        if s.parse::<u64>().unwrap() == curr_day {
            cells.push(Cell::new(s)
                .with_style(Attr::BackgroundColor(color::WHITE))
                .with_style(Attr::ForegroundColor(color::BLACK)));
        } else {
            cells.push(Cell::new(s));
        }
    }
    table.add_row(Row::new(cells));
}
