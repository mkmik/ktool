mod common;

use assert_cmd::Command;

static READ_HUMAN: &str = r#"Message { topic: "topic", partition: 0, offset: 0, timestamp: Some(CreateTime(1663602628526)), headers: "NONE", key: Some("banana-key"), payload: Some("platanos") }"#;
static READ_JSON: &str = r#"{"topic":"topic","partition":0,"offset":0,"timestamp":{"CreateTime":1663602628526},"headers":null,"key":[98,97,110,97,110,97,45,107,101,121],"payload":[112,108,97,116,97,110,111,115]}"#;

macro_rules! assert_output {
    ($output:expr, $matcher:expr) => {
        let got = std::str::from_utf8(&$output).expect("non-unicode bytes in output");
        assert!(
            got.contains($matcher),
            "expected {} in output: {}",
            $matcher,
            got
        )
    };
}

#[test]
fn test_cp() {
    let addr = maybe_skip_integration!();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("cp")
        .arg("./tests/fixture.kbin")
        .arg(format!("kafka://{}/topic", addr));

    let output = cmd.unwrap();

    assert_output!(output.stderr, "read complete");
    assert_output!(output.stderr, "write complete");
    assert_output!(output.stdout, "complete - copied 1 messages");
    assert!(output.status.success());

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("read").arg(format!("kafka://{}/topic", addr));

    let output = cmd.unwrap();
    assert_output!(
        output.stdout,
        r#"key: Some("banana-key"), payload: Some("platanos")"#
    );
}

#[test]
fn test_read_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("read").arg("./tests/fixture.kbin");

    let output = cmd.unwrap();

    assert_output!(output.stderr, "opening dump file");
    assert_output!(output.stdout, READ_HUMAN);
    assert!(output.status.success());
}

#[test]
fn test_read_file_json() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("read").arg("./tests/fixture.kbin").arg("--json");

    let output = cmd.unwrap();

    assert_output!(output.stderr, "opening dump file");
    assert_output!(output.stdout, READ_JSON);
    assert!(output.status.success());
}
