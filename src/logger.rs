use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder};
use sloggers::types::{Severity, Format};

lazy_static! {
    pub static ref LOGGER: Logger = {
        let mut builder = TerminalLoggerBuilder::new();
        builder.format(Format::Compact);
        builder.level(Severity::Debug);

        builder.build().unwrap()
    };
}