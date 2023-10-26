use std::process::Command;

use anyhow::Result;
use assert_fs::prelude::*;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

const BIN_NAME: &str = "puffin";

#[test]
fn missing_pyproject_toml() -> Result<()> {
    let tempdir = assert_fs::TempDir::new()?;
    let pyproject_toml = tempdir.child("pyproject.toml");

    assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
        .arg("add")
        .arg("flask")
        .current_dir(&tempdir));

    pyproject_toml.assert(predicates::path::missing());

    Ok(())
}

#[test]
fn missing_project_table() -> Result<()> {
    let tempdir = assert_fs::TempDir::new()?;
    let pyproject_toml = tempdir.child("pyproject.toml");
    pyproject_toml.touch()?;

    assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
        .arg("add")
        .arg("flask")
        .current_dir(&tempdir));

    pyproject_toml.assert(
        r#"[project]
dependencies = [
    "flask",
]
"#,
    );

    Ok(())
}

#[test]
fn missing_dependencies_array() -> Result<()> {
    let tempdir = assert_fs::TempDir::new()?;
    let pyproject_toml = tempdir.child("pyproject.toml");
    pyproject_toml.touch()?;
    pyproject_toml.write_str(
        r#"[project]
name = "project"
"#,
    )?;

    assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
        .arg("add")
        .arg("flask")
        .current_dir(&tempdir));

    pyproject_toml.assert(
        r#"[project]
name = "project"
dependencies = [
    "flask",
]
"#,
    );

    Ok(())
}

#[test]
fn replace_dependency() -> Result<()> {
    let tempdir = assert_fs::TempDir::new()?;
    let pyproject_toml = tempdir.child("pyproject.toml");
    pyproject_toml.touch()?;
    pyproject_toml.write_str(
        r#"[project]
name = "project"
dependencies = [
    "flask==1.0.0",
]
"#,
    )?;

    assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
        .arg("add")
        .arg("flask==2.0.0")
        .current_dir(&tempdir));

    pyproject_toml.assert(
        r#"[project]
name = "project"
dependencies = [
    "flask==2.0.0",
]
"#,
    );

    Ok(())
}

#[test]
fn reformat_array() -> Result<()> {
    let tempdir = assert_fs::TempDir::new()?;
    let pyproject_toml = tempdir.child("pyproject.toml");
    pyproject_toml.touch()?;
    pyproject_toml.write_str(
        r#"[project]
name = "project"
dependencies = ["flask==1.0.0"]
"#,
    )?;

    assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
        .arg("add")
        .arg("requests")
        .current_dir(&tempdir));

    pyproject_toml.assert(
        r#"[project]
name = "project"
dependencies = [
    "flask==1.0.0",
    "requests",
]
"#,
    );

    Ok(())
}