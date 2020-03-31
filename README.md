# Unofficial extensions for Rust chrono crate.

## Overview

The purpose of this crate is to provide various helpers/extensions for the chrono crate.
 
## Why?

The first requirement that decide me to write this crate was to have "custom" week definitions.
For example, in France, movies are released on Wednesday.

## Features

### Calculate week number with custom week definitions.

French theater calendar have a week definition where the first week has at least 4 days in current year, and starts on Wednesday.
For example:
 
- the first week for 2019 starts on 2019-01-02.
- the first week for 2016 starts on 2015-12-30.

The API for custom week definition is inspired by [Java](https://www.java.com/) API [WeekFields](https://docs.oracle.com/javase/8/docs/api/java/time/temporal/WeekFields.html).
Week is defined by:

- The first day-of-week (Monday, Tuesday, …).
- The minimal number of days in the first week.

Example:

```rust
use chrono::NaiveDate;
use chrono_ext::{WeekSpecification, CustomWeek};

fn use_french_theater_week() {
    let french_theater_week: WeekSpecification = WeekSpecification::french_theater_week();
    let date = NaiveDate::from_ymd(2017, 1, 3);

    let week = french_theater_week.week(date);
    println!("{}", week.format("%Y - W%W")); // 2016 - W53
}
```

## Installation

Add the following to `Cargo.toml` under `[dependencies]`:

```toml
chrono_ext = "0.1.1"
```
