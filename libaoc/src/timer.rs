use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

/// Store several consecutive named time periods
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer {
    /// All named periods of time
    laps: Vec<(Cow<'static, str>, Duration)>,

    /// The time the previous period ended, to be used as the start of the new period.
    previous: Instant,
}

/// Human readable display implementation for Duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DurationFormatter(Duration);

impl Timer {
    /// Create a new timer starting from the current time
    pub fn now() -> Timer {
        Timer {
            laps: vec![],
            previous: Instant::now(),
        }
    }

    /// Record a new named period of time and start the next one.
    /// Starting the next one is no cost and will not be shown when printing
    /// timing results.
    pub fn lap(&mut self, name: impl Into<Cow<'static, str>>) {
        let now = Instant::now();
        let elapsed = now - self.previous;
        self.laps.push((name.into(), elapsed));
        self.previous = now;
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timing Information:")?;

        if self.laps.is_empty() {
            return writeln!(f, " No measured times.");
        } else {
            writeln!(f, "")?;
        }

        // get the longest lap name so the timing table can be nicely formatted
        let width = self.laps.iter().fold(0, |a, (s, _)| a.max(s.len()));

        // get the total amount of time between all periods of time
        let total_time = self.laps.iter().fold(Duration::ZERO, |a, (_, d)| a + *d);
        let total_time = DurationFormatter(total_time);

        for (name, duration) in &self.laps {
            let duration = DurationFormatter(*duration);
            writeln!(f, "  {name:>width$} {duration}")?;
        }

        writeln!(f, "Total: {total_time}")
    }
}

impl Display for DurationFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let DurationFormatter(duration) = self;

        // several seconds, possibly involving minutes
        if duration.as_secs() > 0 {
            // exact number of minutes
            let time = duration.as_secs();
            let min = time - time % 60;

            if min > 1 {
                write!(f, "{min} minutes, ")?;
            } else if min == 1 {
                write!(f, "1 minute, ")?;
            }

            let time = duration.as_secs_f64();
            let sec = time - (min * 60) as f64;
            return write!(f, "{sec:.3}");
        }

        // less than 1 second
        let mut size = "ms";
        let mut time = duration.as_millis();
        if time == 0 {
            size = "us";
            time = duration.as_micros();
        }
        if time == 0 {
            size = "ns";
            time = duration.as_nanos();
        }

        write!(f, "{time}{size}")
    }
}
