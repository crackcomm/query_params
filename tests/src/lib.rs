#[macro_use]
extern crate serde_derive;
extern crate query_params_trait;

#[cfg(test)]
use query_params_trait::QueryParams;

#[test]
fn test_serde() {
    #[derive(Debug, Serialize)]
    enum Enum {
        #[serde(rename = "UNIT")]
        Unit,
    }

    fn serialize_time<S>(time: &std::time::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("T{}h", time.as_secs() / 60 / 60))
    }

    fn serialize_unit<S>(e: &Enum, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("e{:?}", e))
    }

    #[derive(Serialize)]
    struct Test {
        int: u32,
        unit: Enum,
        #[serde(serialize_with = "serialize_time")]
        duration: std::time::Duration,
        #[serde(rename = "unit2", serialize_with = "serialize_unit")]
        custom_unit: Enum,
    }

    let test = Test {
        int: 1,
        unit: Enum::Unit,
        duration: std::time::Duration::from_secs(60 * 60),
        custom_unit: Enum::Unit,
    };
    let expected = r#"int=1&unit=UNIT&duration=T1h&unit2=eUnit"#;
    assert_eq!(test.query_params().unwrap(), expected);
}

#[derive(Serialize)]
struct ExampleStruct {
    pub server: String,
    pub id: i32,
    #[serde(rename = "running")]
    is_running: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
        example_struct.query_params().unwrap(),
        "server=All might&id=42&running=true&tags=latest,linux"
    );
}

#[test]
fn test_ser_empty_vec() {
    let example_struct = ExampleStruct {
        server: "All might".to_string(),
        id: 42,
        is_running: true,
        tags: vec![],
    };

    assert_eq!(
        example_struct.query_params().unwrap(),
        "server=All might&id=42&running=true"
    );
}

#[derive(Serialize)]
struct EmptyStruct {}

#[test]
fn test_ser_for_empty_struct() {
    let empty_struct = EmptyStruct {};

    assert_eq!(empty_struct.query_params().unwrap().len(), 0);
}

#[derive(Serialize)]
struct OptsStruct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pretty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i32>,
}

#[test]
fn test_ser_with_optional_fields() {
    let opts_struct = OptsStruct {
        pretty: Some(true),
        format: Some("json".to_string()),
        depth: None,
    };

    assert_eq!(
        opts_struct.query_params().unwrap(),
        "pretty=true&format=json"
    );
}
