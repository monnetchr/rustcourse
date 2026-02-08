#[derive(Debug)]
struct User {
    username: String,
    active: bool,
}
impl User {
    fn new(username: String) -> User {
        User {
            username,
            active: true,
        }
    }
}
impl User {
    fn clone(&self, active: bool) -> User {
        User {
            username: self.username.clone(),
            active: active,
        }
    }
}
pub fn basics() {
    let strings = [""; 10];
    println!("Strings: {:?}", strings);

    let mut s = String::from("value");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s;
    r3.push_str("-plus");
    println!("r3: {}", r3);

    let mut user = User::new(String::from("user1")).clone(false);
    println!("user: {:#?}, active={}", user, user.active);
    let name = user.username;
    user.username = String::from("user2");
    println!("name: {}, user: {:?}", name, user);
}
