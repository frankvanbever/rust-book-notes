
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
// - Don't have names associated with their fields

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
// - Have no fields. Useful for traits. something in CH10

struct AlwaysEqual;

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

    let black = Color(0,0,0);
    print_color(&black);
    let origin = Point(0,0,0);
    print_point(&origin);

    let _subject = AlwaysEqual;
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

fn print_color(color: &Color) {
    let r = color.0;
    let g = color.1;
    let b = color.2;
    println!("color is {r},{g},{b}");
}

fn print_point(point: &Point) {
    let r = point.0;
    let g = point.1;
    let b = point.2;
    println!("point is {r},{g},{b}");
}
