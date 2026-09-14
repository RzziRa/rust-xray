//! Supervisor-visible bootstrap and fatal logs on stderr (independent of `RUST_LOG`).

use std::borrow::Cow;
use std::process;

use crate::cli::RunOptions;
use crate::config::{config_source_kind, redact_config_source};

/// Write a bootstrap line to stderr (always visible in supervisor `xray.err.log`).
#[macro_export]
macro_rules! eprintln_bootstrap {
    ($($arg:tt)*) => {{
        eprintln!("[rust-xray] {}", ::std::fmt::format(format_args!($($arg)*)))
    }};
}

/// Redact argv for logs (http+unix tokens, query strings).
pub fn redact_argv(args: &[&str]) -> String {
    args.iter()
        .map(|&arg| {
            if arg.contains("http+unix://") || arg.contains("token=") {
                redact_config_source(arg)
            } else {
                Cow::Borrowed(arg)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Early bootstrap for long-running server modes (stderr only; version stays one-line on stdout).
pub fn log_server_bootstrap(raw_args: &[&str], opts: &RunOptions) {
    eprintln_bootstrap!("main_entry start");
    eprintln_bootstrap!("argv: {}", redact_argv(raw_args));
    eprintln_bootstrap!("mode: run");
    eprintln_bootstrap!("config_source_kind: {}", config_source_kind(&opts.config));
    if let Some(format) = opts.format.as_deref() {
        eprintln_bootstrap!("format: {format}");
    } else {
        eprintln_bootstrap!("format: (default)");
    }
    eprintln_bootstrap!("pid: {}", process::id());
    eprintln_bootstrap!("config: {}", redact_config_source(&opts.config));
}

#[cfg(test)]
#[path = "../tests/unit/startup_log.rs"]
mod tests;
