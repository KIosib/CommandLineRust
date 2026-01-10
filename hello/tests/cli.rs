use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("failure");
    assert!(output.status.success());
    let stduot = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert_eq!(stduot, "Hello, world!\n")
}
