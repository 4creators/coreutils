// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
// features ~ feature-gated modules (core/bundler file)

pub mod clap {
    pub use clap::{
        crate_name, crate_version, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, ColorChoice,
        Command, CommandFactory, Error, FromArgMatches, Id, Parser, Subcommand, ValueEnum,
        ValueHint,
    };
    pub mod builder {
        pub use clap::builder::{
            EnumValueParser, NonEmptyStringValueParser, PossibleValue, PossibleValuesParser,
            ValueParser,
        };
        pub use clap_builder::builder::Arg;
    }
    pub mod error {
        pub use clap::error::{ContextKind, ContextValue, ErrorKind};
    }
    pub mod parser {
        pub use clap::parser::{ValueSource, ValuesRef};
    }
}

pub mod regex {
    pub use regex::{Captures, Error, Regex};

    pub mod bytes {
        pub use regex::bytes::Regex;
    }
}
