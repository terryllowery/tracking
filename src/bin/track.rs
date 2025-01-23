use error_stack::{Report, Result, ResultExt};
use tracing::{info, instrument, warn};
use track::{error::{AppError, Suggestion}, init};


fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    Ok(())
    
}