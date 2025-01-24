use error_stack::Result;
use track::{error::AppError, init};

// track is binary name
//
// track start
// track stop
// track report

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    Ok(())
    
}