use error_stack::{Result, ResultExt};
use track::{error::AppError, init, feature::cli};

// track is binary name
//
// track start
// track stop
// track report

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    cli::run().change_context(AppError)
    .attach_printable("a CLI error occurred")?;

    Ok(())
    
}