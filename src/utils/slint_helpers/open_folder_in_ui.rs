use crate::compiled_ui::FsEntryData;
use slint::Image;
use std::fs::DirEntry;
use tokio::io;
use std::path::PathBuf;
use futures_util::FutureExt;
use log::error;
use crate::utils::RawFsEntryData::RawFsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;

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
pub fn open_folder(folder: PathBuf, ctx: AppContext) -> impl Future<Output = ()> {
    let result = tokio::spawn(
        get_entries(
            folder
        )
    );
    result.into_future()
        .then(async |result| {
            match result {
                // TODO: This is temporary, PLEASE USE A POPUP WINDOW, the terminal is useless in a UI app.
                err => error!()
            }
        })
}