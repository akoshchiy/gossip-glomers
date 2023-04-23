use std::sync::Arc;

use maelstrom::{Runtime, Result};

mod handler;

fn main() -> Result<()> {
    Runtime::init(run())
}

async fn run() -> Result<()> {
    let handler = Arc::new(handler::Handler {});
    Runtime::new().with_handler(handler).run().await
}