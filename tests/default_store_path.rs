use todo_cli::path::default_store_path;

#[test]
fn default_store_path_is_under_home_dot_todo_cli() {
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("HOME", dir.path()) };

    let p = default_store_path();
    assert_eq!(p, dir.path().join(".todo_cli").join("todo.json"));
}