use druid::{WindowDesc, AppLauncher};

use carded::build_ui;

fn main() -> Result<(), druid::PlatformError> {
    let main_window = WindowDesc::new(build_ui());
    // DATA
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}
