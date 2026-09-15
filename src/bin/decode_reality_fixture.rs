use clap::Parser;
use rust_xray::reality::{
    decode_reality_fixture_client_hello, reality_fixture_expected_metadata,
    write_reality_fixture_expected_files, RealityFixtureSessionResult,
};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(required = true, value_parser = parse_dir)]
    fixture_dir: PathBuf,
    #[clap(long, short = 'w')]
    write_expected: bool,
    #[clap(long, short = 'f')]
    force: bool,
}

fn parse_dir(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if p.is_dir() {
        Ok(p)
    } else {
        Err(format!("fixture directory '{s}' not found"))
    }
}

fn usage(err: clap::Error) -> ! {
    eprintln!("{err}\n");
    eprintln!("usage: decode_reality_fixture <fixture-dir> [--write-expected] [--force]");
    eprintln!(
        "example: cargo run --bin decode_reality_fixture -- tests/fixtures/reality/basic-xray"
    );
    eprintln!(
        "example: cargo run --bin decode_reality_fixture -- tests/fixtures/reality/basic-xray --write-expected"
    );
    std::process::exit(1);
}

fn parse_cli_args() -> CliArgs {
    CliArgs::try_parse().unwrap_or_else(|e| usage(e))
}

fn read_trimmed(path: &PathBuf) -> std::io::Result<String> {
    Ok(fs::read_to_string(path)?.trim().to_string())
}

fn main() -> ExitCode {
    let cli = parse_cli_args();

    let client_hello_path = cli.fixture_dir.join("client_hello.bin");
    let private_key_path = cli.fixture_dir.join("server_private_key.txt");

    let client_hello = match fs::read(&client_hello_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("failed to read {}: {err}", client_hello_path.display());
            return ExitCode::from(1);
        }
    };

    let private_key = match read_trimmed(&private_key_path) {
        Ok(key) => key,
        Err(err) => {
            eprintln!("failed to read {}: {err}", private_key_path.display());
            return ExitCode::from(1);
        }
    };

    match decode_reality_fixture_client_hello(&client_hello, &private_key) {
        Ok(RealityFixtureSessionResult::Opened {
            sni,
            client_version,
            unix_time,
            short_id_hex,
        }) => {
            println!("REALITY fixture decode OK");
            match &sni {
                Some(hostname) => println!("sni={hostname}"),
                None => println!("sni="),
            }
            println!("client_version={client_version}");
            println!("unix_time={unix_time}");
            println!("short_id={short_id_hex}");
            println!("result=Opened");

            if cli.write_expected {
                let opened = RealityFixtureSessionResult::Opened {
                    sni,
                    client_version,
                    unix_time,
                    short_id_hex,
                };
                match reality_fixture_expected_metadata(&opened).and_then(|metadata| {
                    write_reality_fixture_expected_files(&cli.fixture_dir, &metadata, cli.force)
                }) {
                    Ok(()) => {
                        println!("expected metadata written");
                    }
                    Err(err) => {
                        eprintln!("failed to write expected metadata: {err}");
                        return ExitCode::from(1);
                    }
                }
            }

            ExitCode::SUCCESS
        }
        Ok(RealityFixtureSessionResult::AuthFailed) => {
            eprintln!("REALITY AEAD auth failed");
            println!("result=AuthFailed");
            ExitCode::from(2)
        }
        Err(err) => {
            eprintln!("REALITY fixture decode error: {err}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cli() {
        let args = CliArgs::parse_from([
            "decode_reality_fixture",
            "--fixture-dir",
            "tests/fixtures/reality/basic-xray",
            "--force",
            "--write-expected",
        ]);
        assert!(args.fixture_dir.is_dir());
        assert!(args.write_expected);
        assert!(args.force);
    }
}
