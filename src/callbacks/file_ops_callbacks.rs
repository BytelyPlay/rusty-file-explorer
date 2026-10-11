use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;
use std::rc::Rc;
use log::info;
use crate::utils::slint_helpers::open_folder_in_ui::open_folder;

pub fn fs_entry_clicked(
    entry: FsEntryData,
    ctx: Rc<AppContext>
) {
    open_folder(ctx.get_open_directory().join(entry.name), ctx);
}