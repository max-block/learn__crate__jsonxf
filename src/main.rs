use serde_json::json;

fn main() {
    let data = json!({"a": 1, "b": "ddd", "c": true, "d": [1,2,3,4,5]});
    let r = jsonxf::pretty_print(&data.to_string()).unwrap();
    println!("{}", r);
}
