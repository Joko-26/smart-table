mod types;

pub struct User {
    id: String,
    name: String,
    plan: String,
    devices: Dict<String>,
}
