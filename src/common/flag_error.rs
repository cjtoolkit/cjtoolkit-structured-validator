pub fn flag_error<T, E>(flag: &mut bool, result: Result<T, E>) -> Result<T, E> {
    if result.is_err() {
        *flag = true;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_error() {
        let mut flag = false;
        let result: Result<(), ()> = flag_error(&mut flag, Ok(()));
        assert!(result.is_ok());
        assert!(!flag);
    }

    #[test]
    fn test_flag_error_err() {
        let mut flag = false;
        let result: Result<(), ()> = flag_error(&mut flag, Err(()));
        assert!(result.is_err());
        assert!(flag);
    }
}
