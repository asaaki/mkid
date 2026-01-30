use assert_cmd::Command;
use uuid::Uuid;

#[allow(deprecated)]
fn get_cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn test_default_generates_v7() {
    let mut cmd = get_cmd();
    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    // Verify it's a valid UUID
    let uuid = Uuid::parse_str(uuid_str).expect("Should be valid UUID");

    // Verify it's v7
    assert_eq!(uuid.get_version(), Some(uuid::Version::SortRand));
}

#[test]
fn test_uuid_v4() {
    let mut cmd = get_cmd();
    let output = cmd.args(["uuid", "v4"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    let uuid = Uuid::parse_str(uuid_str).expect("Should be valid UUID");
    assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
}

#[test]
fn test_uuid_v5_deterministic() {
    let mut cmd1 = get_cmd();
    let output1 = cmd1
        .args(["uuid", "v5", "--namespace", "dns", "--name", "example.com"])
        .assert()
        .success();
    let uuid1 = String::from_utf8(output1.get_output().stdout.clone()).unwrap();

    let mut cmd2 = get_cmd();
    let output2 = cmd2
        .args(["uuid", "v5", "--namespace", "dns", "--name", "example.com"])
        .assert()
        .success();
    let uuid2 = String::from_utf8(output2.get_output().stdout.clone()).unwrap();

    assert_eq!(uuid1, uuid2, "v5 should be deterministic");
}

#[test]
fn test_format_simple() {
    let mut cmd = get_cmd();
    let output = cmd
        .args(["uuid", "v7", "--format", "simple"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    // Simple format has no hyphens
    assert!(!uuid_str.contains('-'));
    assert_eq!(uuid_str.len(), 32);
}

#[test]
fn test_format_urn() {
    let mut cmd = get_cmd();
    let output = cmd
        .args(["uuid", "v7", "--format", "urn"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    assert!(uuid_str.starts_with("urn:uuid:"));
}

#[test]
fn test_format_braced() {
    let mut cmd = get_cmd();
    let output = cmd
        .args(["uuid", "v7", "--format", "braced"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    assert!(uuid_str.starts_with('{'));
    assert!(uuid_str.ends_with('}'));
}

#[test]
fn test_uppercase() {
    let mut cmd = get_cmd();
    let output = cmd.args(["uuid", "v7", "--uppercase"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let uuid_str = stdout.trim();

    // Check that it contains uppercase letters
    assert!(uuid_str.chars().any(|c| c.is_ascii_uppercase()));
    // Verify no lowercase letters (except the hyphens)
    assert!(
        !uuid_str
            .chars()
            .filter(|c| c.is_alphabetic())
            .any(|c| c.is_ascii_lowercase())
    );
}

#[test]
fn test_count() {
    let mut cmd = get_cmd();
    let output = cmd.args(["uuid", "v7", "--count", "5"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines.len(), 5);

    // Verify all are valid UUIDs
    for line in lines {
        Uuid::parse_str(line).expect("Should be valid UUID");
    }
}

#[test]
fn test_v7_monotonic() {
    let mut cmd = get_cmd();
    let output = cmd.args(["uuid", "v7", "--count", "10"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    let uuids: Vec<Uuid> = lines
        .iter()
        .map(|line| Uuid::parse_str(line).unwrap())
        .collect();

    // Verify they are sorted (monotonic)
    for i in 1..uuids.len() {
        assert!(uuids[i] >= uuids[i - 1], "v7 UUIDs should be monotonic");
    }
}

#[test]
fn test_error_v5_missing_name() {
    let mut cmd = get_cmd();
    cmd.args(["uuid", "v5", "--namespace", "dns"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("--name is required"));
}

#[test]
fn test_error_v8_invalid_bytes() {
    let mut cmd = get_cmd();
    cmd.args(["uuid", "v8", "--bytes", "123"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("32 hex characters"));
}
