use std::path::{Path, PathBuf};

use slint_build::CompileError;

fn main() {
    compile_all_slint().expect(".slint files were unable to compile")
}

fn compile_all_slint() -> Result<(), CompileError> {
    const UI_FOLDER: &'static str = "ui"; 
    // TODO: Figure out how to do this without hardcoding the ui folder, use a constant for the ui folder
    const MAIN_SLINT_FILE: &'static str = "ui/AppWindow.slint";

    let conf = 
        slint_build::CompilerConfiguration::new()
        .with_include_paths(
            vec![UI_FOLDER.into()]
        );
    
    slint_build::compile_with_config(
        MAIN_SLINT_FILE, 
        conf
    )?;

    Ok( () )
}
