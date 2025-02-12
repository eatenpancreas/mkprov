// use mkprov_lsib::workspace::Workspace;

// use super::query::ItemKind;

// fn temp_workdir_create() -> Workspace {
//     let p = tempdir::TempDir::new("").unwrap().into_path();
//     let mut wk = Workspace::custom_create(p).unwrap();

//     let mut source_dir = std::env::current_dir().unwrap();
//     source_dir.pop();
//     source_dir.push("test_files/eu4");
//     wk.game_location = source_dir;

//     wk
// }

// #[test]
// fn test_prov_query() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["100".into()], wk);
//     assert_eq!(out[0].to_string(), "province:100");
// }

// #[test]
// fn test_prov_name_query() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["Friesland".into()], wk);
//     assert_eq!(out[0].to_string(), "province:100");
// }

// #[test]
// fn test_prov_name_weird() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["Trøndelag".into()], wk);
//     assert_eq!(out[0].to_string(), "province:20");
// }

// #[test]
// fn test_prov_name_weird_2() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["trondelag".into()], wk);
//     assert!(out[0].is_sure());
//     assert_eq!(out[0].to_string(), "province:20")
// }

// #[test]
// fn test_prov_name_weird_3() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["trudelog".into()], wk);
//     assert!(!out[0].is_sure());
// }

// #[test]
// fn test_prov_name_weird_4() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["hdusdsdu".into()], wk);
//     assert!(!out[0].is_sure());
// }

// #[test]
// fn test_prov_name_weird_5() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["holm".into()], wk);
//     assert!(!out[0].is_sure());
// }

// #[test]
// fn test_prov_name_weird_6() {
//     let wk = temp_workdir_create();
//     assert!(wk.game_location.exists());

//     let out = QueryArgs::main(ItemKind::Province, vec!["olland".into()], wk);
//     assert!(!out[0].is_sure());
// }
