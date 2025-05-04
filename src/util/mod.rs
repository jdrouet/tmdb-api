pub(crate) mod empty_string;

pub(crate) fn is_false(value: &bool) -> bool {
    !*value
}
