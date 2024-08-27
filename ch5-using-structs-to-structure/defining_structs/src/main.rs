
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("user@example.com"),
        String::from("username123"));

    print_user(&user1);

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("user2"),
        ..user1
    };

    print_user(&user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    let mut foo: &str = "✅";
    if user.active == false {
        foo = "❎";
    }

    let name = &user.username;
    let mail = &user.email;
    let count = user.sign_in_count;
    println!("{name} - {foo}");
    println!("    email: {mail}");
    println!("    sign-in count: {count}");

}
