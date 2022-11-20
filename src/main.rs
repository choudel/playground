#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}
fn main() {
    let username = "patt";
    let user = find_user(username).map(|user_id| User {
        user_id,
        name: username.to_owned(),
    });
    println!("{user:?}");
    match user {
        Some(user) => println!("{user:?}"),
        None => println!("user not found"),
    }
}
