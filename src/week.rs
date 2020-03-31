use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::error::Error;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct WeekSpecification {
    first_day: Weekday,
    min_days_in_first_week: u32,
}

impl WeekSpecification {
    pub fn new(first_day: Weekday, min_days_in_first_week: u32) -> Result<WeekSpecification, Error> {
        if min_days_in_first_week < 1 || min_days_in_first_week > 7 {
            Err(Error::OutOfRange(min_days_in_first_week, 1, 7))
        } else {
            Ok(WeekSpecification {
                first_day,
                min_days_in_first_week,
            })
        }
    }

    pub fn sunday_start() -> WeekSpecification {
        WeekSpecification {
            first_day: Weekday::Sun,
            min_days_in_first_week: 1,
        }
    }

    pub fn iso_week() -> WeekSpecification {
        WeekSpecification {
            first_day: Weekday::Mon,
            min_days_in_first_week: 4,
        }
    }

    pub fn french_theater_week() -> WeekSpecification {
        WeekSpecification {
            first_day: Weekday::Wed,
            min_days_in_first_week: 4,
        }
    }

    pub fn first_day(&self) -> Weekday {
        self.first_day
    }

    pub fn min_days_in_first_week(&self) -> u32 {
        self.min_days_in_first_week
    }

    /// Find the first day of week based year for current specification.
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_ext::WeekSpecification;
    ///
    /// let iso_week: WeekSpecification = WeekSpecification::iso_week();
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// assert_eq!(NaiveDate::from_ymd(2018, 12, 31), iso_week.first_day_of_week_based_year(2019));
    /// assert_eq!(NaiveDate::from_ymd(2019, 1, 2), french_theater_week.first_day_of_week_based_year(2019));
    /// ~~~~
    pub fn first_day_of_week_based_year(&self, year: i32) -> NaiveDate {
        let reference = NaiveDate::from_ymd(year, 1, self.min_days_in_first_week);

        let january_first = NaiveDate::from_ymd(year, 1, 1);
        let delta = 7 - self.num_days_from_first_dow(january_first.weekday());
        // delta is between 0 and 6. So, "with_ordinal0" result cannot be invalid.
        let week_date = january_first.with_ordinal0(delta).unwrap();

        if week_date <= reference {
            week_date
        } else {
            week_date - Duration::weeks(1)
        }
    }

    /// Find the last day of week based year for current specification.
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_ext::WeekSpecification;
    ///
    /// let iso_week: WeekSpecification = WeekSpecification::iso_week();
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// assert_eq!(NaiveDate::from_ymd(2019, 12, 29), iso_week.last_day_of_week_based_year(2019));
    /// assert_eq!(NaiveDate::from_ymd(2019, 12, 31), french_theater_week.last_day_of_week_based_year(2019));
    /// ~~~~
    pub fn last_day_of_week_based_year(&self, year: i32) -> NaiveDate {
        self.first_day_of_week_based_year(year + 1) - Duration::days(1)
    }

    /// Count year weeks for current week specification.
    ///
    /// ~~~~
    /// use chrono_ext::WeekSpecification;
    ///
    /// let iso_week: WeekSpecification = WeekSpecification::iso_week();
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// assert_eq!(52, iso_week.num_weeks(2019));
    /// assert_eq!(52, french_theater_week.num_weeks(2019));
    /// assert_eq!(53, french_theater_week.num_weeks(2016));
    /// ~~~~
    pub fn num_weeks(&self, year: i32) -> u32 {
        let diff = self.last_day_of_week_based_year(year) - self.first_day_of_week_based_year(year);
        1 + (diff.num_days() as u32 / 7)
    }

    /// Number of days from current week (from 0 to 6).
    ///
    /// ~~~~
    /// use chrono::Weekday;
    /// use chrono_ext::WeekSpecification;
    ///
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// assert_eq!(0, french_theater_week.num_days_from_first_dow(Weekday::Wed));
    /// assert_eq!(6, french_theater_week.num_days_from_first_dow(Weekday::Tue));
    /// ~~~~
    pub fn num_days_from_first_dow(&self, day: Weekday) -> u32 {
        (7 + day.num_days_from_monday() - self.first_day.num_days_from_monday()) % 7
    }

    /// Number of the day from current week specification (from 1 to 7).
    ///
    /// ~~~~
    /// use chrono::Weekday;
    /// use chrono_ext::WeekSpecification;
    ///
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// assert_eq!(1, french_theater_week.number_from_first_dow(Weekday::Wed));
    /// assert_eq!(7, french_theater_week.number_from_first_dow(Weekday::Tue));
    /// ~~~~
    pub fn number_from_first_dow(&self, day: Weekday) -> u32 {
        1 + self.num_days_from_first_dow(day)
    }

    /// Compute week for a given date, according current specification.
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_ext::{WeekSpecification, CustomWeek};
    ///
    /// let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    ///
    /// let french_theater_first_dow_2016_53 = NaiveDate::from_ymd(2016, 12, 28);
    /// let french_theater_last_dow_2016_53 = NaiveDate::from_ymd(2017, 1, 3);
    ///
    /// let week = french_theater_week.week(french_theater_last_dow_2016_53);
    ///
    /// assert_eq!(2016, week.year());
    /// assert_eq!(53, week.week());
    /// assert_eq!(52, week.week0());
    /// assert_eq!(french_theater_first_dow_2016_53, week.week_start());
    /// assert_eq!(french_theater_week, week.specification());
    /// ~~~~
    pub fn week(&self, date: NaiveDate) -> CustomWeek {
        let date_year = date.year();
        let first = self.first_day_of_week_based_year(date_year);
        let last = self.last_day_of_week_based_year(date_year);

        let (year, week) = if date < first {
            // Last week of year - 1
            (date_year - 1, self.num_weeks(date_year - 1))
        } else if date > last {
            // First week of year + 1
            (date_year + 1, 1)
        } else {
            let diff = date - first;
            (date_year, 1 + (diff.num_days() as u32 / 7))
        };

        let week_start = date - chrono::Duration::days(self.num_days_from_first_dow(date.weekday()) as i64);

        CustomWeek {
            year,
            week,
            week_start,
            specification: self.clone(),
        }
    }
}


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
