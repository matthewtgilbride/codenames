mod game;

// hello world lambda handler
use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;
use simple_logger::SimpleLogger;

#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();
    lambda!(handler);

    Ok(())
}

fn handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}
