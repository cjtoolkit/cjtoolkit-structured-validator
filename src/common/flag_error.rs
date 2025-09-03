pub fn flag_error<T, E>(flag: &mut bool, result: Result<T, E>) -> Result<T, E> {
    if result.is_err() {
        *flag = true;
    }
    result
}
