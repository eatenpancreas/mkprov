use mkprov_eu4::{
    common::ItemKind,
    query::{QueryArgs, QueryError},
};
use mkprov_lib::workspace::Workspace;

fn temp_workdir_create() -> Workspace {
    let p = tempdir::TempDir::new("").unwrap().into_path();
    let mut wk = Workspace::custom_create(p).unwrap();

    let mut source_dir = std::env::current_dir().unwrap();
    source_dir.pop();
    source_dir.push("test_files/eu4");
    wk.game_location = Some(source_dir);

    wk
}

#[test]
fn test_workdir_exists() {
    let workspace = temp_workdir_create();
    assert!(workspace.game_location_exists());
}

#[test]
fn test_prov_query() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["100".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert_eq!(output, vec![100]);
}

#[test]
fn test_prov_query_empty() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec![] }
        .query_province_ids(&temp_workdir_create());

    assert_eq!(output, Err(QueryError::ItemsEmpty));
}

#[test]
fn test_prov_name_query() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["Friesland".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert_eq!(output, vec![100]);
}

#[test]
fn test_prov_name_weird() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["Trøndelag".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert_eq!(output, vec![20]);
}

#[test]
fn test_prov_name_weird_2() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["trondelag".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert_eq!(output, vec![20]);
}

#[test]
fn test_prov_name_weird_3() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["trudelag".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert_eq!(output, vec![20]);
}

#[test]
fn test_prov_name_weird_4() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["hdsudududu".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert!(output.is_empty());
}

#[test]
fn test_prov_name_weird_5() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["holm".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert!(output.is_empty());
}

#[test]
fn test_prov_name_weird_6() {
    let output = QueryArgs { kind: ItemKind::Province, items: vec!["olland".into()] }
        .query_province_ids(&temp_workdir_create())
        .unwrap();

    assert!(output.is_empty());
}
