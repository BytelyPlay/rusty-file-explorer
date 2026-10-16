use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::app_context_wrapper::AppContextWrapper;
use std::fmt::Display;

pub fn display_error_in_result_async<T, E: Display + Send + 'static>(
    result: Result<T, E>,
    wrapped_ctx: AppContextWrapper,
) -> Option<T> {
    if let Ok(val) = result {
        return Some(val);
    }

    let err = result.err().unwrap();

    wrapped_ctx.upgrade_on_slint_event_loop(
        move |ctx| create_error_popup(
            err,
            ctx
        )
    ).expect("EventLoopError thrown in display_error_in_result_async");

    None
}

/// This should show a dialog popup, if the result is an error.
/// If it isn't an error, it returns the value inside of the Result.
///
/// But at the moment, it just panics if there is an error.
pub fn display_error_in_result<T, E: Display>(
    result: Result<T, E>,
    ctx: AppContext
) -> Option<T> {
    if result.is_ok() {
        return result.ok();
    }

    create_error_popup(result.err().unwrap(), ctx);

    None
}

fn create_error_popup(err: impl Display, ctx: AppContext) {
    ctx.show_error_popup(
        format!("{}", err)
    );
}