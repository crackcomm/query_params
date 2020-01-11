#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate query_params_derive;
extern crate query_params_trait;

#[cfg(test)]
use query_params_trait::QueryParams;

#[derive(QueryParams, Serialize)]
struct ExampleStruct {
    pub server: String,
    pub id: i32,
    #[query(rename = "running")]
    #[serde(default)]
    is_running: bool,
    tags: Vec<String>,
}

#[test]
fn test_ser_query_params_with_primitive_types() {
    let example_struct = ExampleStruct {
        server: "All might".to_string(),
        id: 42,
        is_running: true,
        tags: vec!["latest".to_string(), "linux".to_string()],
    };

    assert_eq!(
        example_struct.query_params(),
        "server=All might&id=42&running=true&tags=latest,linux"
    );
}

#[derive(QueryParams)]
struct EmptyStruct {}

#[test]
fn test_ser_for_empty_struct() {
    let empty_struct = EmptyStruct {};

    assert_eq!(empty_struct.query_params().len(), 0);
}

#[derive(QueryParams)]
struct OptsStruct {
    pretty: Option<bool>,
    format: Option<String>,
    depth: Option<i32>,
}

#[test]
fn test_ser_with_optional_fields() {
    let opts_struct = OptsStruct {
        pretty: Some(true),
        format: Some("json".to_string()),
        depth: None,
    };

    assert_eq!(opts_struct.query_params(), "pretty=true&format=json");
}
