use serde_json::json;
use serde_json::Value;

pub async fn get_all_users() -> Value {
    // 模拟数据库数据
    json!([
        { "id": 1, "name": "Alice" },
        { "id": 2, "name": "Bob" }
    ])
}
