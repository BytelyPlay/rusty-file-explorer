use log::info;
use slint::SharedString;
use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;

pub fn fill_initial_files(weak_ctx: std::rc::Weak<AppContext>) {
    if let Some(ctx) = weak_ctx.upgrade() {
        for _i in 1..100 {
            ctx.get_files().push(FsEntryData {
                icon: Default::default(),
                name: SharedString::from(
                    "Some cool file"
                ),
            });
            println!("{}", ctx.get_files().get(0).unwrap().name)
        }
    }
}