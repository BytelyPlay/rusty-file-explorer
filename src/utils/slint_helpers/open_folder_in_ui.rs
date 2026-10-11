use crate::compiled_ui::FsEntryData;
use crate::utils::raw_fs_entry_data::RawFsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::display_error::display_error_in_result;
use futures_util::FutureExt;
use slint::Image;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::rc::Rc;
use tokio::io;

/// Returns the first I/O error it gets, if it gets an error, otherwise it opens the folder.
pub async fn get_entries(folder: PathBuf) -> io::Result<Vec<RawFsEntryData>> {
    const DEFAULT_FILE_NAME: &str = "\
    File name couldn't be converted into a normal String\
    ";

    let all_entries: Vec<io::Result<DirEntry>> = std::fs::read_dir(folder)?
        .collect();
    let mut fs_entries: Vec<RawFsEntryData> = vec![];

    for entry in all_entries {
        let os_str = entry?.file_name();
        let os_str_as_str_result = os_str.to_str();

        let file_name: String;

        match os_str_as_str_result {
            Some(str) => file_name = str.to_string(),
            None => file_name = DEFAULT_FILE_NAME.to_string()
        }

        fs_entries.push(
            RawFsEntryData {
                name: file_name
            }
        );
    }

    Ok(fs_entries)
}
/// This opens a folder and handles any error that may appear.
/// At the moment, it just panics, but later on it will show a popup window that shows the error.
pub fn open_folder(folder: PathBuf, ctx: Rc<AppContext>) {
    let wrapper = ctx.as_wrapper();

    let future = get_entries(folder)
        .then(async move |result| {
            if let Some(data) = display_error_in_result(result) {
                display_error_in_result(
                    wrapper.upgrade_on_slint_event_loop(
                        // TODO: Update the open_directory thing.
                        |ctx| fill_file_state(
                            data, ctx
                        )
                    )
                );
            }
        });
    tokio::spawn(future);
}

fn fill_file_state(data: Vec<RawFsEntryData>, ctx: AppContext) {
    ctx.get_files().set_vec(data.iter()
        .map(|raw| {
            FsEntryData {
                icon: Image::default(),
                name: raw.name.clone().into()
            }
        })
        .collect()
    )
}