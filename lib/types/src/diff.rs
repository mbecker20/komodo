use diff::{Diff, HashMapDiff, OptionDiff, VecDiff};

use crate::deployment::{DockerRunArgsDiff, RestartModeDiff};

pub fn option_diff_no_change<T: Diff>(option_diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    option_diff == &OptionDiff::NoChange || option_diff == &OptionDiff::None
}

pub fn vec_diff_no_change<T: Diff>(vec_diff: &VecDiff<T>) -> bool {
    vec_diff.0.is_empty()
}

pub fn hashmap_diff_no_change<T: Diff>(hashmap_diff: &HashMapDiff<String, T>) -> bool {
    hashmap_diff.altered.is_empty() && hashmap_diff.removed.is_empty()
}

pub fn docker_run_args_diff_no_change(dra: &DockerRunArgsDiff) -> bool {
    dra.image.is_none()
        && option_diff_no_change(&dra.container_user)
        && option_diff_no_change(&dra.docker_account)
        && option_diff_no_change(&dra.network)
        && option_diff_no_change(&dra.post_image)
        && vec_diff_no_change(&dra.environment)
        && vec_diff_no_change(&dra.ports)
        && vec_diff_no_change(&dra.volumes)
        && restart_mode_diff_no_change(&dra.restart)
}

pub fn restart_mode_diff_no_change(restart_mode: &RestartModeDiff) -> bool {
    restart_mode == &RestartModeDiff::NoChange
}
