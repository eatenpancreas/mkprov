use mod_workspace::Workspace;

#[test]
fn test_workspace_loading() {
    let pb = std::env::current_dir().unwrap();
    let i = pb.into_iter().last().unwrap().to_str().unwrap();
    assert_eq!(i, "mod_workspace");

    let wk = Workspace::load().unwrap().unwrap();
    let file = wk.get_string_file("a");

    let _ = file.load().unwrap();
}
