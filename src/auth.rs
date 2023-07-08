use std::io;
use crate::models::User;

pub fn login() -> Result<User, Box<dyn std::error::Error>> {
    let mut username = String::new();
    let mut password = String::new();

    println!("Please enter your username:");
    io::stdin().read_line(&mut username)?;
    println!("Please enter your password:");
    io::stdin().read_line(&mut password)?;

    username = username.trim().to_string();
    password = password.trim().to_string();

    let user = User::find_by_username(username.to_string())?;

    if user.authenticate(password) {
        println!("User is {:?} authenticated", user);
        return Ok(user);
    }

    println!("User is not authenticated");
    Ok(User::guest())
}
