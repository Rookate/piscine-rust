use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let y = year as i32;

    let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    if is_leap {
        return None;
    }

    let date = NaiveDate::from_yo_opt(y, 183)?;
    Some(date.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", middle_day(1023));
    }
}
