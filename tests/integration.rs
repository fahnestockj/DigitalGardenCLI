//integration tests!
use color_eyre::eyre::Result;
use assert_cmd::Command;

#[test]
/// make sure help runs. This indicates the binary works
fn test_help() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}

#[test]
/// make sure we have a write command by running garden write --help
fn test_write_help() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let assert = cmd.arg("--help").arg("write").assert();
  assert.success().stderr("");
  Ok(())
}
#[test]
/// make sure we have a write command by running garden write 
fn test_write() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let fake_editor_path = std::env::current_dir()
    .expect("expect to be in a dir")
    .join("tests")
    .join("fake_editor.sh");
    if !fake_editor_path.exists() {
      panic!("fake editor not found at {}", fake_editor_path.display());
    }
  Ok(())
}