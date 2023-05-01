use diff::{Diff, OptionDiff, VecDiff};

use crate::{
    deployment::{DockerRunArgsDiff, RestartModeDiff, TerminationSignalDiff},
    TimelengthDiff,
};

pub fn f64_diff_no_change(f64_diff: &f64) -> bool {
    *f64_diff == 0.0
}

pub fn f32_diff_no_change(f32_diff: &f32) -> bool {
    *f32_diff == 0.0
}

pub fn i32_diff_no_change(i32_diff: &i32) -> bool {
    *i32_diff == 0
}

pub fn option_diff_no_change<T: Diff>(option_diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    option_diff == &OptionDiff::NoChange || option_diff == &OptionDiff::None
}

pub fn vec_diff_no_change<T: Diff>(vec_diff: &VecDiff<T>) -> bool {
    vec_diff.0.is_empty()
}

// pub fn hashmap_diff_no_change<K: Hash + Eq, T: Diff>(hashmap_diff: &HashMapDiff<K, T>) -> bool {
//     hashmap_diff.altered.is_empty() && hashmap_diff.removed.is_empty()
// }

pub fn docker_run_args_diff_no_change(dra: &DockerRunArgsDiff) -> bool {
    dra.image.is_none()
        && dra.network.is_none()
        && option_diff_no_change(&dra.container_user)
        && option_diff_no_change(&dra.docker_account)
        && option_diff_no_change(&dra.post_image)
        && vec_diff_no_change(&dra.environment)
        && vec_diff_no_change(&dra.ports)
        && vec_diff_no_change(&dra.volumes)
        && vec_diff_no_change(&dra.extra_args)
        && restart_mode_diff_no_change(&dra.restart)
}

pub fn restart_mode_diff_no_change(restart_mode: &RestartModeDiff) -> bool {
    restart_mode == &RestartModeDiff::NoChange
}

pub fn timelength_diff_no_change(timelength: &TimelengthDiff) -> bool {
    timelength == &TimelengthDiff::NoChange
}

pub fn termination_signal_diff_no_change(term_signal: &TerminationSignalDiff) -> bool {
    term_signal == &TerminationSignalDiff::NoChange
}
