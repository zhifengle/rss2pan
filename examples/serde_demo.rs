use serde_json::Value;

fn main() {
    let mut v: Value = serde_json::from_str("{}").unwrap();
    println!("{:?}", v["not_exist"]);
    v["test"] = "aa".into();
    println!("{:?}", v);
}
