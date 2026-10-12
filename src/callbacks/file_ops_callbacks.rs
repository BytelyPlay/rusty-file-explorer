use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;
use std::rc::Rc;
use log::info;
use crate::utils::slint_helpers::open_folder_in_ui::open_folder;

pub fn fs_entry_clicked(
    entry: &FsEntryData,
    ctx: Rc<AppContext>
) {
    let open_dir = ctx.get_open_directory();
    let joined_open_dir = open_dir.join(entry.name.clone());

    open_folder(joined_open_dir, ctx);
}