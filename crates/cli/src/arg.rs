use clap::{Parser, Subcommand, ValueEnum};

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Options {
    /// Custom config file
    #[clap(short, long)]
    pub config: Option<String>,

    /// Query
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Notify by message push service
    #[clap(short, long)]
    pub notify: bool,

    /// Verbose
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// API Queries
    Query {
        /// Selection
        #[clap(value_enum)]
        query: Query,

        /// Argument
        arg: String,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Query {
    /// Query Electricity by UID
    #[clap(name = "ele")]
    Electricity,

    /// Query UID by phone number
    #[clap(name = "uid")]
    Uid,
}
