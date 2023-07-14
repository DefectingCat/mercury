use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Registry,
};

use crate::error::MResult;

pub fn logger_init() -> MResult<()> {
    let formattin_layer = fmt::layer()
        .pretty()
        .with_thread_ids(true)
        // .with_target(false)
        .with_file(false)
        .with_writer(std::io::stdout);

    let filter = filter::LevelFilter::DEBUG;
    Registry::default()
        .with(filter)
        .with(ErrorLayer::default())
        .with(formattin_layer)
        .init();

    Ok(())
}
