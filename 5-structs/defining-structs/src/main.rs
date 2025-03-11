/// Struct can store multiple fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


/// Tuple struct. Looks like a tuple, but requires the struct name
struct Color(u32, u32, u32);
struct Point(i32, i32, i32);

fn main() {
    // We can define an instance of the struct
    let mut user1 = User {  // Can only define the whole struct as mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");

    // We can then create an instance, using fields from user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    // Or a shorthand! `...` is the *update syntax* 
    // This does through an error though. Sad.
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

/// Function to construct an instance of the User struct
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}
