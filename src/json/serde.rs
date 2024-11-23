// https://crates.io/crates/serde

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[test]
    fn json_example() {
        let obj= serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        let serialized = serde_json::to_string(&obj).unwrap();
        println!("serialized = {}", serialized);

        let str = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let deserialized: serde_json::Value = serde_json::from_str(str).unwrap();
        println!("deserialized = {:?}", deserialized);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn serialize_point() {
        let point = Point { x: 1, y: 2 };

        let serialized = serde_json::to_string(&point).unwrap();
        println!("serialized = {}", serialized);
        assert_eq!(serialized, r#"{"x":1,"y":2}"#);
    }

    #[test]
    fn deserialize_point() {
        let deserialized: Point = serde_json::from_str(r#"{"x":1,"y":2}"#).unwrap();
        println!("deserialized = {:?}", deserialized);
        assert_eq!(deserialized, Point { x: 1, y: 2 });
    }
}
