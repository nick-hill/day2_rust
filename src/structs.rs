struct User {
    username : String,
    email : String,
    gender : char,
    is_active : bool,
}

impl User {
    fn print(&self) {
        println!("Name of the user is:  {}", self.username);
        println!("Email of the user is: {}", self.email);
        println!("Gender of the user is: {}", self.gender);
        println!("Is user active? {}", self.is_active);
    }
}

pub fn run() {

    let user1 = initialize();
    user1.print();
}

fn initialize() -> User {
    println!("Enter username: ");
    let username: String = read!("{}\n");

    println!("Enter email of the user: ");
    let email: String = read!("{}\n");

    println!("Enter gender of the user: ");
    let gender: char = read!("{}");

    println!("Is user active: ");
    let is_active: bool = read!("{}");

    print!("\n");

    User {
        username,
        email,
        gender,
        is_active,
    }
}
