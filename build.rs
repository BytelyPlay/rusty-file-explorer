use slint_build::CompileError;
use constcat::concat;

fn main() {
    compile_all_slint().expect(".slint files were unable to compile")
}

fn compile_all_slint() -> Result<(), CompileError> {
    const UI_FOLDER: &str = "ui"; 
    const MAIN_SLINT_FILE: &str = concat!(
        UI_FOLDER, "/main-window.slint"
    );

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
