#[allow(unused)]
const SAMPLE_PUZZLE_INPUT: &'static str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

pub(crate) fn second_december() {
    let mut safe_reports = 0;

    for report_input in include_str!("second-part1.txt").lines() {
        // parse report
        let report: Vec<u8> = report_input
            .split_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        if report_can_be_safe(&report) {
            safe_reports += 1;
        }
    }

    dbg!(safe_reports);
}

fn report_is_safe(report: &Vec<u8>) -> bool {
    #[derive(PartialEq, Eq, Clone, Copy)]
    enum ReportState {
        IsIncreasing,
        IsDecreasing,
        Unknown,
    }

    let mut previous_level: Option<u8> = None;
    let mut detected_report_state = ReportState::Unknown;

    for level in report {
        if let Some(previous_level) = previous_level {
            let abs_diff = level.abs_diff(previous_level);

            if abs_diff < 1 || abs_diff > 3 {
                eprintln!("any two adjacent levels differ by at least one and at most three, got {abs_diff}!");
                return false; // diff needs to be greater than equal 1 but smaller than 4
            }

            let current_trend = if *level > previous_level {
                ReportState::IsIncreasing
            } else {
                ReportState::IsDecreasing
            };

            if detected_report_state == ReportState::Unknown {
                detected_report_state = current_trend;
            }

            if current_trend != detected_report_state {
                eprintln!("not monotonically increasing or decreasing!");
                return false;
            }
        }

        previous_level = Some(*level);
    }

    true
}

fn report_can_be_safe(report: &Vec<u8>) -> bool {
    if report_is_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if report_is_safe(&modified_report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::second::{report_can_be_safe, report_is_safe};

    #[test]
    fn safe_report_1() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(report_is_safe(&report));
    }

    #[test]
    fn unsafe_report_2() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!report_is_safe(&report));
    }

    #[test]
    fn unsafe_report_3() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!report_is_safe(&report));
    }

    #[test]
    fn unsafe_report_4() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(!report_is_safe(&report));
    }

    #[test]
    fn unsafe_report_5() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(!report_is_safe(&report));
    }

    #[test]
    fn safe_report_6() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(report_is_safe(&report));
    }

    #[test]
    fn can_be_made_safe_1() {
        assert!(report_can_be_safe(&vec![1, 3, 2, 4, 5]));
    }

    #[test]
    fn can_be_made_safe_2() {
        assert!(report_can_be_safe(&vec![8, 6, 4, 4, 1]));
    }
}
