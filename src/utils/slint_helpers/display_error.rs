use std::error::Error;
use futures_util::future::err;
use log::warn;
use slint::{ComponentHandle, ToSharedString};
use crate::compiled_ui::ErrorDialog;
use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::app_context_wrapper::AppContextWrapper;

/// This should show a dialog popup, if the result is an error.
/// If it isn't an error, it returns the value inside of the Result.
///
/// But at the moment, it just panics if there is an error.
pub fn display_error_in_result<T, E: Error>(
    result: Result<T, E>, ctx: AppContextWrapper
) -> Option<T> {
    if result.is_ok() {
        return result.ok();
    }

    create_error_dialog(result.err().unwrap())

    None
}

fn create_error_dialog(err: impl Error) {
    // I don't know of a better way to handle this error.
    let error_dialog_result =
        ErrorDialog::new();

    if let Err(err) = error_dialog_result {
        panic!("Error occurred in display_error_in_result(): {}", err);
    }
    let error_dialog = error_dialog_result.unwrap();
    let weak_error_dialog = error_dialog.as_weak();

    error_dialog.on_ok_clicked(move || {
        let dialog_result = weak_error_dialog.upgrade();

        if dialog_result.is_none() {
            warn!(
                "ok_clicked callback called on ErrorDialog, \
                but couldn't upgrade the weak pointer."
            );
        }
        dialog_result.unwrap().hide().expect("Couldn't hide ErrorDialog");
    });
    error_dialog.set_error_message(
        format!("{}", err).into()
    );
}