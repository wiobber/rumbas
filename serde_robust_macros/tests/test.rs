use serde::Deserialize;
use serde_robust::create_value_type;
use serde_robust::Error;
use serde_robust_macros::Deserialize_robust;

create_value_type!(Value);

mod small_prime {
    use super::*;

    #[derive(Deserialize, Deserialize_robust, PartialEq, Debug)]
    struct ExamplePart {
        field1: usize,
        field2: usize,
    }

    #[derive(Deserialize, Deserialize_robust, PartialEq, Debug)]
    struct Example {
        data: ExamplePart,
    }
    #[derive(Deserialize, Deserialize_robust, Debug)]
    struct RobustExample {
        data: Value<ExamplePart>,
    }

    /*
    #[test]
    fn test_serialize() {
        let j = serde_json::to_string(&SmallPrime::Seven).unwrap();
        assert_eq!(j, "7");
    }*/

    #[test]
    fn test_deserialize() {
        let p: Value<Example> =
            serde_json::from_str(r#"{"data": {"field1": 1, "field2": 2}}"#).unwrap();
        assert_eq!(
            p.0.unwrap(),
            Example {
                data: ExamplePart {
                    field1: 1,
                    field2: 2
                }
            }
        );
        let p: Value<Example> =
            serde_json::from_str(r#"{"data":{"field3": 1, "field2": 2}}"#).unwrap();
        assert_eq!(
            p.0.unwrap(),
            Example {
                data: ExamplePart {
                    field1: 1,
                    field2: 2
                }
            }
        );
        //let p: SmallPrime = serde_json::from_str("Two").unwrap();
        //assert_eq!(p, SmallPrime::Two);
    }
    #[test]
    fn test_deserialize2() {
        let p: Value<RobustExample> =
            serde_json::from_str(r#"{"data": {"field1": 1, "field2": 2}}"#).unwrap();
        println!("{:?}", p);
        let p: Value<RobustExample> =
            serde_json::from_str(r#"{"data":{"field3": 1, "field2": 2}}"#).unwrap();
        panic!("{:?}", p);
        //let p: SmallPrime = serde_json::from_str("Two").unwrap();
        //assert_eq!(p, SmallPrime::Two);
    }
}
