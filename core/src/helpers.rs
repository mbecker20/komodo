use diff::{Diff, OptionDiff};
use types::Log;

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn option_diff_is_some<T: Diff>(diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    diff != &OptionDiff::NoChange && diff != &OptionDiff::None
}

pub fn any_option_diff_is_some<T: Diff>(diffs: &[&OptionDiff<T>]) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    for diff in diffs {
        if diff != &&OptionDiff::NoChange && diff != &&OptionDiff::None {
            return true;
        }
    }
    return false;
}

pub fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}
