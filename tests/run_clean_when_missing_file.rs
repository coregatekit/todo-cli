use clap::Parser;
use todo_cli::Cli;

#[test]
fn clean_works_when_store_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "clean"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "No items to clear\n");
}