//! pentazygonic calendar - the Five Great Months of the Pentazygon
// UMODK... 0(::9), 1(::8), 2(::7), 3(::6), 4(::5)
// MOK(DU) 1, 2, 3, 8, 7, 5 (0, 3, 6, 9)
// psz: 5 * 72(=9) = 365
use std::time::{SystemTime, UNIX_EPOCH};

const DAYS_IN_YEAR: u32 = 365;
const MONTHS_IN_YEAR: u32 = 5;
const FIVE_GREAT_MONTHS: u32 = 5;
const DAYS_IN_MONTH: u32 = 73;
const SECONDS_IN_YEAR: u64 = 60 * 60 * 24 * 73 * 5;
const SECONDS_IN_MONTH: u64 = SECONDS_IN_YEAR / 5;
const SECONDS_IN_DAY: u64 = 60 * 60 * 24;

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
    println!("DMY: {} {} {}", curr.day, curr.month, curr.year);

    format_date(&curr);
}

fn format_date(curr: &Curr) {
    println!("MDY");
}
