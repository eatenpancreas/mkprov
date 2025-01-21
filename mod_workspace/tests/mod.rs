use mod_workspace::Workspace;

#[test]
fn test_workspace_current() {
    let pb = std::env::current_dir().unwrap();
    let i = pb.into_iter().last().unwrap().to_str().unwrap();
    assert_eq!(i, "mod_workspace");
    assert!(Workspace::load().unwrap().is_none());
}
