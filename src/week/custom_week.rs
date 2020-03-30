use crate::week::week_specification::WeekSpecification;
use chrono::{NaiveDate, Duration};

/// Custom week implementation.
///
/// PartialOrd and Ord are not implemented, because we do not have a natural ordering for weeks with different specifications.
/// Week start could be recalculated from year, week and specification: it is only stored by convenience for `succ`, `pred`, and `contains` functions.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomWeek {
    year: i32,
    week: u32,
    week_start: NaiveDate,
    specification: WeekSpecification,
}

impl CustomWeek {
    pub fn new(year: i32, week: u32, week_start: NaiveDate, specification: WeekSpecification) -> CustomWeek {
        CustomWeek {
            year,
            week,
            week_start,
            specification,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn week(&self) -> u32 {
        self.week
    }

    /// Week number in year (0 based).
    pub fn week0(&self) -> u32 {
        self.week - 1
    }

    pub fn week_start(&self) -> NaiveDate {
        self.week_start
    }

    pub fn specification(&self) -> WeekSpecification {
        self.specification
    }

    /// The next week according to the same specification.
    pub fn succ(&self) -> CustomWeek {
        self.specification.week(self.week_start + Duration::weeks(1))
    }

    /// The previous week according to the same specification.
    pub fn pred(&self) -> CustomWeek {
        self.specification.week(self.week_start - Duration::weeks(1))
    }

    /// Verify if the given date is in the current week.
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.week_start && date < (self.week_start + Duration::weeks(1))
    }

    /// Very naive week formatting
    ///
    /// Formatters are inspired by `chrono::format::strftime`.
    ///
    /// | Spec. | Example  | Description                                            |
    /// |-------|----------|--------------------------------------------------------|
    /// |       |          | **DATE SPECIFIERS:**                                   |
    /// | `%Y`  | `2001`   | The week year, zero-padded to 4 digits.                |
    /// | `%C`  | `20`     | The week year divided by 100, zero-padded to 2 digits. |
    /// | `%y`  | `01`     | The week year modulo 100, zero-padded to 2 digits.     |
    /// | `%W`  | `27`     | Week number, zero-padded to 2 digits.                  |
    /// |-------|----------|--------------------------------------------------------|
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_ext::{WeekSpecification, CustomWeek};
    ///
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// let french_theater_dow_2016_53 = NaiveDate::from_ymd(2017, 1, 3);
    /// let week = french_theater_week.week(french_theater_dow_2016_53);
    ///
    /// assert_eq!("Year 2016", week.format("Year %Y"));
    /// assert_eq!("Year 2016", week.format("Year %C%y"));
    /// assert_eq!("Week 53", week.format("Week %W"));
    /// assert_eq!("S1653", week.format("S%y%W"));
    /// ~~~~
    ///
    pub fn format(&self, fmt: &str) -> String {
        let full_year = format!("{:04}", self.year);
        let y_div_100 = format!("{:02}", self.year / 100);
        let y_mod_100 = format!("{:02}", self.year % 100);
        let week = format!("{:02}", self.week);

        fmt
            .replace("%Y", &full_year)
            .replace("%C", &y_div_100)
            .replace("%y", &y_mod_100)
            .replace("%W", &week)
    }
}
