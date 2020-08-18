use std::env;

mod internal {
    //! Internal configuration options.

    /// ### const PREFIX
    /// Holds the command prefix for the bot.
    pub const PREFIX: &'static str = "!";

    /// ### const VERSION
    /// Holds the current version number.
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

/// ### struct Config
/// Holds configuration data for the bot.
#[derive(Debug, Clone)]
pub struct Config {
    /// Discord API token
    token: String,
    /// Command prefix
    prefix: &'static str,
    /// Version number
    version: &'static str,
}

impl Config {
    /// ### fn new() -> Self
    /// Creates a configuration instance.
    pub fn new() -> Self {
        Config {
            token: env::var("DISCORD_TOKEN").expect("No env token"),
            prefix: internal::PREFIX,
            version: internal::VERSION,
        }
    }

    /// ### fn token() -> &'static str
    /// Returns the API token.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// ### fn prefix() -> &'static str
    /// Returns the command prefix.
    pub fn prefix(&self) -> &'static str {
        self.prefix
    }

    /// ### fn version() -> &'static str
    /// Returns the current version.
    #[allow(dead_code)]
    pub fn version(&self) -> &'static str {
        self.version
    }
}
