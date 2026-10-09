use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;
use std::rc::Rc;
use log::info;

pub fn fs_entry_clicked(
    entry: FsEntryData,
    ctx: Rc<AppContext>
) {
    info!("{}", entry.name)
}