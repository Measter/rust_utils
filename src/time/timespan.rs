use std::time::Duration;

const NANOS_PER_MILLISECOND_F: f64 = 1_000_000.0;
const NANOS_PER_SECOND_F: f64 = 1_000_000_000.0;
const NANOS_PER_MILLISECOND: u32 = 1_000_000;
const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * 60;
const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;

/// Trait is based on .Net's [`TimeSpan`](https://docs.microsoft.com/en-us/dotnet/api/system.timespan?view=netframework-4.7) type.
pub trait TimeSpan<T> {
    /// Returns the days part of the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.partial_days(), 5);
    /// ```
    fn partial_days(&self) -> u64;
    /// Returns the hours part of the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.partial_hours(), 7);
    /// ```
    fn partial_hours(&self) -> u8;
    /// Returns the minutes part of the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.partial_minutes(), 34);
    /// ```
    fn partial_minutes(&self) -> u8;
    /// Returns the seconds part of the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.partial_seconds(), 15);
    /// ```
    fn partial_seconds(&self) -> u8;
    /// Returns the milliseconds part of the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.partial_milliseconds(), 236);
    /// ```
    fn partial_milliseconds(&self) -> u16;

    /// Returns the total number of days, whole and fractional, represented by the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// assert_eq!(span.total_days(), 5.31545413);
    /// ```
    fn total_days(&self) -> f64;
    /// Returns the total number of hours, whole and fractional, represented by the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// // Round to precision because of the inaccuracies in floating point maths.
    /// let span = (span.total_hours() * 1_000_000.0).round() / 1_000_000.0;
    /// assert_eq!(span, 127.570899);
    /// ```
    fn total_hours(&self) -> f64;
    /// Returns the total number of minutes, whole and fractional, represented by the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// // Round to precision because of the inaccuracies in floating point maths.
    /// let span = (span.total_minutes() * 1_0000.0).round() / 1_0000.0;
    /// assert_eq!(span, 7654.2539);
    /// ```
    fn total_minutes(&self) -> f64;
    /// Returns the total number of seconds, whole and fractional, represented by the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// // Round to precision because of the inaccuracies in floating point maths.
    /// let span = (span.total_seconds() * 1_000.0).round() / 1_000.0;
    /// assert_eq!(span, 459255.237);
    /// ```
    fn total_seconds(&self) -> f64;
    /// Returns the total number of milliseconds, whole and fractional, represented by the time span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let span = Duration::from_total_days(5.31545413).unwrap();
    /// // Round to precision because of the inaccuracies in floating point maths.
    /// let span = span.total_milliseconds().round();
    /// assert_eq!(span, 459255237.0);
    /// ```
    fn total_milliseconds(&self) -> f64;

    /// Returns a timespan representing the given number of days.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let week = Duration::from_total_days(7.0);
    /// ```
    fn from_total_days(days: f64) -> Result<T, String>;
    /// Returns a timespan representing the given number of hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let hours = Duration::from_total_hours(13.543);
    /// ```
    fn from_total_hours(hours: f64) -> Result<T, String>;
    /// Returns a timespan representing the given number of minutes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let minutes = Duration::from_total_minutes(20.0);
    /// ```
    fn from_total_minutes(minutes: f64) -> Result<T, String>;
    /// Returns a timespan representing the given number of seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let seconds = Duration::from_total_seconds(13.5);
    /// ```
    fn from_total_seconds(seconds: f64) -> Result<T, String>;
    /// Returns a timespan representing the given number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let milliseconds = Duration::from_total_milliseconds(516.0);
    /// ```
    fn from_total_milliseconds(milliseconds: f64) -> Result<T, String>;

    /// Returns a timespan representing the given number of days.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let week = Duration::from_days(7);
    /// ```
    fn from_days(days: u64) -> T;
    /// Returns a timespan representing the given number of hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let hours = Duration::from_hours(13);
    /// ```
    fn from_hours(hours: u64) -> T;
    /// Returns a timespan representing the given number of minutes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let minutes = Duration::from_minutes(20);
    /// ```
    fn from_minutes(minutes: u64) -> T;
    /// Returns a timespan representing the given number of seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let seconds = Duration::from_seconds(13);
    /// ```
    fn from_seconds(seconds: u64) -> T;
    /// Returns a timespan representing the given number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use rust_utils::time::TimeSpan;
    /// 
    /// let milliseconds = Duration::from_milliseconds(516);
    /// ```
    fn from_milliseconds(milliseconds: u64) -> T;
}

macro_rules! input_check {
    ($val:expr) => (
        if $val.is_sign_negative() || $val.is_nan() || $val.is_infinite() {
            return Err(format!("Invalid timespan: {:?}", $val));
        }
    )
}

impl TimeSpan<Duration> for Duration {
    fn partial_days(&self) -> u64 {
        (self.as_secs() / SECONDS_PER_DAY) as u64
    }
    fn partial_hours(&self) -> u8 {
        let secs = self.as_secs() % SECONDS_PER_DAY;
        (secs / SECONDS_PER_HOUR) as u8
    }
    fn partial_minutes(&self) -> u8 {
        let secs = (self.as_secs() % SECONDS_PER_DAY) % SECONDS_PER_HOUR;
        (secs / SECONDS_PER_MINUTE) as u8
    }
    fn partial_seconds(&self) -> u8 {
        let secs = ((self.as_secs() % SECONDS_PER_DAY) % SECONDS_PER_HOUR) % SECONDS_PER_MINUTE;
        secs as u8
    }
    fn partial_milliseconds(&self) -> u16{
        (self.subsec_nanos() / NANOS_PER_MILLISECOND) as u16
    }

    fn total_days(&self) -> f64 {
        let total_days = self.as_secs() as f64 / SECONDS_PER_DAY as f64;
        let total_nanoseconds = self.subsec_nanos() as f64 / NANOS_PER_SECOND_F / SECONDS_PER_DAY as f64;
        total_days + total_nanoseconds
    }
    fn total_hours(&self) -> f64 {
        let total_hours = self.as_secs() as f64 / SECONDS_PER_HOUR as f64;
        let total_nanoseconds = self.subsec_nanos() as f64 / NANOS_PER_SECOND_F / SECONDS_PER_HOUR as f64;
        total_hours + total_nanoseconds
    }
    fn total_minutes(&self) -> f64 {
        let total_minutes = self.as_secs() as f64 / SECONDS_PER_MINUTE as f64;
        let total_nanoseconds = self.subsec_nanos() as f64 / NANOS_PER_SECOND_F / SECONDS_PER_MINUTE as f64;
        total_minutes + total_nanoseconds
    }
    fn total_seconds(&self) -> f64 {
        let total_seconds = self.as_secs() as f64;
        let total_nanoseconds = self.subsec_nanos() as f64 / NANOS_PER_SECOND_F;
        total_seconds + total_nanoseconds
    }
    fn total_milliseconds(&self) -> f64 {
        let total_milliseconds = (self.as_secs() * 1000) as f64;
        let total_nanoseconds = self.subsec_nanos() as f64 / NANOS_PER_MILLISECOND_F;
        total_milliseconds + total_nanoseconds
    }

    fn from_total_days(days: f64) -> Result<Duration, String> {
        input_check!(days);

        let days_in_sec = days * SECONDS_PER_DAY as f64;
        let full_days_in_sec = days_in_sec.trunc() as u64;
        let frac_days_in_sec = (days_in_sec.fract() * NANOS_PER_SECOND_F).round() as u32;

        Ok(Duration::new(full_days_in_sec, frac_days_in_sec))
    }
    fn from_total_hours(hours: f64) -> Result<Duration, String> {
        input_check!(hours);

        let hours_in_sec = hours * SECONDS_PER_HOUR as f64;
        let full_hours_in_sec = hours_in_sec.trunc() as u64;
        let frac_hours_in_sec = (hours_in_sec.fract() * NANOS_PER_SECOND_F).round() as u32;

        Ok(Duration::new(full_hours_in_sec, frac_hours_in_sec))
    }
    fn from_total_minutes(minutes: f64) -> Result<Duration, String> {
        input_check!(minutes);

        let minutes_in_sec = minutes * SECONDS_PER_MINUTE as f64;
        let full_minutes_in_sec = minutes_in_sec.trunc() as u64;
        let frac_minutes_in_sec = (minutes_in_sec.fract() * NANOS_PER_SECOND_F).round() as u32;

        Ok(Duration::new(full_minutes_in_sec, frac_minutes_in_sec))
    }
    fn from_total_seconds(seconds: f64) -> Result<Duration, String> {
        input_check!(seconds);

        let full_seconds_in_sec = seconds.trunc() as u64;
        let frac_seconds_in_sec = (seconds.fract() * NANOS_PER_SECOND_F).round() as u32;

        Ok(Duration::new(full_seconds_in_sec, frac_seconds_in_sec))
    }
    fn from_total_milliseconds(milliseconds: f64) -> Result<Duration, String> {
        input_check!(milliseconds);

        let milliseconds_in_nano_sec = (milliseconds * NANOS_PER_MILLISECOND_F).round() as u32;

        Ok(Duration::new(0, milliseconds_in_nano_sec))
    }

    fn from_days(days: u64) -> Duration {
        Duration::new(days * SECONDS_PER_DAY, 0)
    }
    fn from_hours(hours: u64) -> Duration {
        Duration::new(hours * SECONDS_PER_HOUR, 0)
    }
    fn from_minutes(minutes: u64) -> Duration {
        Duration::new(minutes * SECONDS_PER_MINUTE, 0)
    }
    fn from_seconds(seconds: u64) -> Duration {
        Duration::new(seconds, 0)
    }
    fn from_milliseconds(milliseconds: u64) -> Duration {
        let secs = milliseconds / 1000;
        let nanos = (milliseconds % 1000) as u32 * NANOS_PER_MILLISECOND;

        Duration::new(secs, nanos)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::f64;
    use super::TimeSpan;

    #[test]
    fn input_negative() {
        let neg = Duration::from_total_days(-4.0);

        assert!(neg.is_err());
    }

    #[test]
    fn input_infinite() {
        let inf = Duration::from_total_days(f64::INFINITY);

        assert!(inf.is_err());
    }

    #[test]
    fn input_nan() {
        let nan = Duration::from_total_days(f64::NAN);

        assert!(nan.is_err());
    }


    #[test]
    fn from_total_days_two_weeks() {
        let span = Duration::from_total_days(14.0).unwrap();

        assert_eq!(span, Duration::new(1209600, 0));
    }

    #[test]
    fn from_total_days_one_and_half_day() {
        let span = Duration::from_total_days(1.5).unwrap();

        assert_eq!(span, Duration::new(129600,0));
    }

    #[test]
    fn from_total_days_one_and_third() {
        let span = Duration::from_total_days(1.3333).unwrap();

        assert_eq!(span, Duration::new(115197,120_000_000));
    }


    #[test]
    fn from_total_hours_two_hours() {
        let span = Duration::from_total_hours(2.0).unwrap();

        assert_eq!(span, Duration::new(7200, 0));
    }

    #[test]
    fn from_total_hours_one_and_half_hours() {
        let span = Duration::from_total_hours(1.5).unwrap();

        assert_eq!(span, Duration::new(5400,0));
    }

    #[test]
    fn from_total_hours_one_and_third() {
        let span = Duration::from_total_hours(1.3333).unwrap();

        assert_eq!(span, Duration::new(4799,880_000_000));
    }


    #[test]
    fn from_total_minutes_two_minutes() {
        let span = Duration::from_total_minutes(2.0).unwrap();

        assert_eq!(span, Duration::new(120, 0));
    }

    #[test]
    fn from_total_minutes_one_and_half_minutes() {
        let span = Duration::from_total_minutes(1.5).unwrap();

        assert_eq!(span, Duration::new(90,0));
    }

    #[test]
    fn from_total_minutes_one_and_third() {
        let span = Duration::from_total_minutes(1.3333).unwrap();

        assert_eq!(span, Duration::new(79,998_000_000));
    }


    #[test]
    fn from_total_seconds_two_seconds() {
        let span = Duration::from_total_seconds(2.0).unwrap();

        assert_eq!(span, Duration::new(2, 0));
    }

    #[test]
    fn from_total_seconds_one_and_half_seconds() {
        let span = Duration::from_total_seconds(1.5).unwrap();

        assert_eq!(span, Duration::new(1,500_000_000));
    }

    #[test]
    fn from_total_seconds_one_and_third() {
        let span = Duration::from_total_seconds(1.3333).unwrap();

        assert_eq!(span, Duration::new(1,333_300_000));
    }


    #[test]
    fn from_total_milliseconds_two_milliseconds() {
        let span = Duration::from_total_milliseconds(2.0).unwrap();

        assert_eq!(span, Duration::new(0, 2_000_000));
    }

    #[test]
    fn from_total_milliseconds_one_and_half_milliseconds() {
        let span = Duration::from_total_milliseconds(1.5).unwrap();

        assert_eq!(span, Duration::new(0, 1_500_000));
    }

    #[test]
    fn from_total_milliseconds_one_and_third() {
        let span = Duration::from_total_milliseconds(1.3333).unwrap();

        assert_eq!(span, Duration::new(0, 1_333_300));
    }


    #[test]
    fn from_days_two_weeks() {
        let span = Duration::from_days(14);

        assert_eq!(span, Duration::new(1209600, 0));
    }

    #[test]
    fn from_hours_two_hours() {
        let span = Duration::from_hours(2);

        assert_eq!(span, Duration::new(7200, 0));
    }

    #[test]
    fn from_minutes_two_minutes() {
        let span = Duration::from_minutes(2);

        assert_eq!(span, Duration::new(120, 0));
    }

    #[test]
    fn from_seconds_two_seconds() {
        let span = Duration::from_seconds(2);

        assert_eq!(span, Duration::new(2, 0));
    }

    #[test]
    fn from_milliseconds_two_milliseconds() {
        let span = Duration::from_milliseconds(2);

        assert_eq!(span, Duration::new(0, 2_000_000));
    }



    #[test]
    fn partial_days() {
        let span = Duration::from_total_days(1.51354973541463).unwrap();

        assert_eq!(span.partial_days(), 1);
    }

    #[test]
    fn partial_hours() {
        let span = Duration::from_total_days(1.51354973541463).unwrap();

        assert_eq!(span.partial_hours(), 12);
    }

    #[test]
    fn partial_minutes() {
        let span = Duration::from_total_days(1.51354973541463).unwrap();

        assert_eq!(span.partial_minutes(), 19);
    }

    #[test]
    fn partial_seconds() {
        let span = Duration::from_total_days(1.51354973541463).unwrap();

        assert_eq!(span.partial_seconds(), 30);
    }

    #[test]
    fn partial_milliseconds() {
        let span = Duration::from_total_days(1.51354973541463).unwrap();

        assert_eq!(span.partial_milliseconds(), 697);
    }

    #[test]
    fn total_days() {
        let span = Duration::from_total_days(1.5135497354).unwrap();

        // Round to precision because of the inaccuracies in floating point maths.
        let span = (span.total_days() * 1_000_000_000_0.0).round() / 1_000_000_000_0.0;

        assert_eq!(span, 1.5135497354);
    }

    #[test]
    fn total_hours() {
        let span = Duration::from_total_days(1.5135497354).unwrap();

        // Round to precision because of the inaccuracies in floating point maths.
        let span = (span.total_hours() * 1_000_000_00.0).round() / 1_000_000_00.0;

        assert_eq!(span, 36.32519365);
    }

    #[test]
    fn total_minutes() {
        let span = Duration::from_total_days(1.5135497354).unwrap();

        // Round to precision because of the inaccuracies in floating point maths.
        let span = (span.total_minutes() * 1_000_000.0).round() / 1_000_000.0;

        assert_eq!(span, 2179.511619);
    }

    #[test]
    fn total_seconds() {
        let span = Duration::from_total_days(1.5135497354).unwrap();

        // Round to precision because of the inaccuracies in floating point maths.
        let span = (span.total_seconds() * 1_0000.0).round() / 1_0000.0;

        assert_eq!(span, 130770.6971);
    }

    #[test]
    fn total_milliseconds() {
        let span = Duration::from_total_days(1.5135497354).unwrap();

        // Round to precision because of the inaccuracies in floating point maths.
        let span = (span.total_milliseconds() * 1_0.0).round() / 1_0.0;

        assert_eq!(span, 130770697.1);
    }
}
