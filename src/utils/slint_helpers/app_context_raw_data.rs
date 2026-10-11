use std::path::PathBuf;

/// This is basically just the data for the AppContext, that is Send and isn't referenced-counted or something.
/// For example: strings or integers.
pub struct AppContextRawData {
    pub open_path: PathBuf
}