use assert_cmd::Command;
use predicates::prelude::*;

fn taskgraph() -> Command {
    Command::cargo_bin("taskgraph").unwrap()
}

#[test]
fn test_list_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("task-one"))
        .stdout(predicate::str::contains("task-two"))
        .stdout(predicate::str::contains("task-three"));
}

#[test]
fn test_list_with_status_filter() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("list")
        .arg("--status")
        .arg("completed")
        .assert()
        .success()
        .stdout(predicate::str::contains("task-three"))
        .stdout(predicate::str::contains("task-one").not());
}

#[test]
fn test_show_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("show")
        .arg("task-one")
        .assert()
        .success()
        .stdout(predicate::str::contains("First Task"));
}

#[test]
fn test_show_missing_task() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("show")
        .arg("missing-task")
        .assert()
        .failure();
}

#[test]
fn test_validate_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("validate")
        .assert()
        .success();
}

#[test]
fn test_topo_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("topo")
        .assert()
        .success()
        .stdout(predicate::str::contains("task-one"));
}

#[test]
fn test_deps_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("deps")
        .arg("task-two")
        .assert()
        .success()
        .stdout(predicate::str::contains("task-one"));
}

#[test]
fn test_dependents_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("dependents")
        .arg("task-one")
        .assert()
        .success()
        .stdout(predicate::str::contains("task-two"))
        .stdout(predicate::str::contains("task-three"));
}

#[test]
fn test_cycles_command_no_cycles() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("cycles")
        .assert()
        .success()
        .stdout(predicate::str::contains("No cycles"));
}

#[test]
fn test_parallel_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("parallel")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generation"));
}

#[test]
fn test_critical_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("critical")
        .assert()
        .success();
}

#[test]
fn test_graph_command() {
    taskgraph()
        .arg("-p")
        .arg("tests/fixtures/tasks")
        .arg("graph")
        .assert()
        .success()
        .stdout(predicate::str::contains("digraph"));
}

#[test]
fn test_help_flag() {
    taskgraph()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI tool for managing task dependencies",
        ));
}

#[test]
fn test_version_flag() {
    taskgraph().arg("--version").assert().success();
}
