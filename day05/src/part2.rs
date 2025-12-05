use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut intervals: Vec<Interval> = Vec::new();

    for line in cont.lines() {
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        let interval = Interval {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        };

        intervals.push(interval);
    }

    let mut collapsed_intervals: Vec<Interval> = Vec::new();
    intervals.sort_by(|a, b| b.start.cmp(&a.start));
    loop {
        if intervals.len() == 1 {
            collapsed_intervals.push(intervals[0].clone());
            break;
        }
        let interval = intervals.pop().unwrap();
        let test_interval = intervals.pop().unwrap();
        if interval.overlaps(&test_interval) {
            let collapsed = interval.collapse(&test_interval);
            intervals.push(collapsed);
        } else {
            intervals.push(test_interval);
            collapsed_intervals.push(interval);
        }
    }

    let count = collapsed_intervals
        .iter()
        .map(|interval| interval.size())
        .sum::<i64>();
    println!("{}", count);
}

#[derive(Debug, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn overlaps(&self, other: &Interval) -> bool {
        return !(other.end < self.start || self.end < other.start);
    }
    fn collapse(&self, other: &Interval) -> Interval {
        return Interval {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        };
    }
    fn size(&self) -> i64 {
        return self.end - self.start + 1;
    }
}
