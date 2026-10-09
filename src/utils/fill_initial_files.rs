use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::open_folder::open_folder;

pub fn fill_initial_files(ctx: &AppContext) {
    open_folder(dirs::home_dir().unwrap(), ctx).await.unwrap();
}