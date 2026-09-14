use super::*;

#[test]
fn redact_argv_hides_token_in_http_unix_uri() {
    let args = vec![
        "rw-core",
        "-config",
        "http+unix:///run/a.sock/internal/get-config?token=secret",
        "-format",
        "json",
    ];
    let redacted = redact_argv(&args);
    assert!(!redacted.contains("secret"));
    assert!(redacted.contains("?<redacted>"));
}
