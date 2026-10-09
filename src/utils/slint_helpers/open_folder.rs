use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;
use slint::Image;
use crate::compiled_ui::FsEntryData;
use crate::utils::slint_helpers::app_context::AppContext;

/// Returns the first I/O error it gets, if it gets an error. Otherwise it opens the folder.
pub async fn open_folder(folder: PathBuf, ctx: &AppContext) -> io::Result<()> {
    const DEFAULT_FILE_NAME: &str = "\
    File name couldn't be converted into a normal String\
    ";

    let all_entries: Vec<io::Result<DirEntry>> = std::fs::read_dir(folder)?
        .collect();
    let mut fs_entries: Vec<FsEntryData> = vec![];

    for entry in all_entries {
        let os_str = entry?.file_name();
        let os_str_as_str_result = os_str.to_str();

        let file_name: String;

        match os_str_as_str_result {
            Some(str) => file_name = str.to_string(),
            None => file_name = DEFAULT_FILE_NAME.to_string()
        }
        fs_entries.push(
            FsEntryData {
                icon: Image::default(),
                name: file_name.into()
            }
        )
    }
    ctx.
    ctx.get_files()
        .set_vec(fs_entries);
    
    Ok( () )
}