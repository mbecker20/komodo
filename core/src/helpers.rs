use std::str::FromStr;

use anyhow::anyhow;
use diff::{Diff, OptionDiff};
use helpers::to_monitor_name;
use types::Build;

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

pub fn parse_comma_seperated_list<T: FromStr>(comma_sep_list: &str) -> anyhow::Result<Vec<T>> {
    comma_sep_list
        .split(",")
        .filter(|item| item.len() > 0)
        .map(|item| {
            let item = item
                .parse()
                .map_err(|_| anyhow!("error parsing string {item} into type T"))?;
            Ok::<T, anyhow::Error>(item)
        })
        .collect()
}

pub fn get_image_name(build: &Build) -> String {
    let name = to_monitor_name(&build.name);
    match &build.docker_organization {
        Some(org) => format!("{org}/{name}"),
        None => match &build.docker_account {
            Some(acct) => format!("{acct}/{name}"),
            None => name,
        },
    }
}
