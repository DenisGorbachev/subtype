pub fn get_result<T, E>(option: Option<E>, value: T) -> Result<T, E> {
    match option {
        None => Ok(value),
        Some(error) => Err(error),
    }
}
