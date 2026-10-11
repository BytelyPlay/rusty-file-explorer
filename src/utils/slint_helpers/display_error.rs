use std::error::Error;

/// This should make a popup window, if the result is an error.
/// If it isn't an error, it returns the value inside of the Result.
///
/// But at the moment, it just panics if there is an error.
pub fn display_error_in_result<T, E: Error>(result: Result<T, E>) -> Option<T> {
    Some(result.unwrap())
}