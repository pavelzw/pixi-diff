use pixi_diff_cli::{Input, diff};
use std::path::Path;

#[test]
fn test_diff() {
    let before = Input::File("tests/resources/pixi.lock.old".into());
    let after = Input::File("tests/resources/pixi.lock.new".into());

    let json = diff(before, after, None).unwrap();

    let expected = std::fs::read_to_string(Path::new("tests/resources/diff.json")).unwrap();
    let expected = expected.trim();

    assert_eq!(json, expected);
}
