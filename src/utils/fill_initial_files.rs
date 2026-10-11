use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::open_folder_in_ui::open_folder;
use std::rc::Rc;

pub fn fill_initial_files(ctx: Rc<AppContext>) {
    let _ = open_folder(dirs::home_dir().unwrap(), ctx);
}