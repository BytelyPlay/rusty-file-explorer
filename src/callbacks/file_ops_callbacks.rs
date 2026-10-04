use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;
use std::rc::Rc;
use log::info;

pub fn fs_entry_clicked(
    _fs_entry: FsEntryData,
    _main_window: Rc<AppContext>
) {
    info!("log")
}