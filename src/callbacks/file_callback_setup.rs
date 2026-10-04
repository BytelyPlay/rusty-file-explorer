use std::rc::Rc;
use log::{error, warn};
use crate::callbacks::file_ops_callbacks;

use crate::compiled_ui::{FsEntryData};
use crate::utils::slint_helpers::app_context::AppContext;

pub fn setup_callbacks(weak_ctx: std::rc::Weak<AppContext>) {
    if let Some(ctx) = weak_ctx.upgrade() {
        setup_callbacks_internal(ctx.clone());
    } else {
        error!("For some reason, weak pointer to the main window was not able to be upgraded. \
        Cannot setup callbacks.");
    }
}

fn setup_callbacks_internal(ctx: Rc<AppContext>) {
    let weak_ctx = Rc::downgrade(&ctx);

    let closure = move |fs_entry: FsEntryData| {
        let opt_app_context = weak_ctx.upgrade();

        if let Some(app_context) = opt_app_context {
            file_ops_callbacks::fs_entry_clicked(
                fs_entry, app_context.clone()
            );
        }
    };
    ctx.get_callbacks().on_fs_entry_clicked(
        closure
    );
}

fn ctx_as_strong_or_log(
    weak_ctx: std::rc::Weak<AppContext>
) -> Option<Rc<AppContext>> {
    let opt_ctx = weak_ctx.upgrade();

    if opt_ctx.is_none() {
        warn!("Couldn't upgrade Weak<AppContext> while calling callback\
        , this could mean that the \
        callback was somehow called after the AppContext went out of scope.")
    }
    opt_ctx
}