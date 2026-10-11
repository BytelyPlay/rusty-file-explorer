use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::open_folder_in_ui::get_entries;

pub fn fill_initial_files(ctx: AppContext) {
    
    get_entries(dirs::home_dir().unwrap())
}