use std::io;

use strum::IntoEnumIterator;
pub fn prompt_for_deck_details() -> Result<(String, Option<String>), io::Error> {
    let mut name = String::new();
    let mut description = String::new();

    println!("Name: ");
    io::stdin().read_line(&mut name)?;

    println!("Description: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut description)?;

    Ok((
        name.trim().to_string(),
        Some(description.trim().to_string()),
    ))
}
pub fn prompt_for_deck_id() -> Result<i64, io::Error> {
    let mut id = String::new();

    println!("ID: ");
    io::stdin().read_line(&mut id)?;
    Ok(id.trim().parse().unwrap())
}
pub fn prompt_for_card_details() -> Result<(Option<String>, Option<String>), io::Error> {
    let mut front = String::new();
    let mut back = String::new();

    println!("Front: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut front)?;

    println!("Back: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut back)?;

    match (front.trim(), back.trim()) {
        ("", "") => Ok((None, None)),
        (front, "") => Ok((Some(front.to_string()), None)),
        ("", back) => Ok((None, Some(back.to_string()))),
        (front, back) => Ok((Some(front.to_string()), Some(back.to_string()))),
    }
}

pub fn prompt_for_card_id() -> Result<i64, io::Error> {
    let mut id = String::new();
    println!("ID: ");
    io::stdin().read_line(&mut id)?;
    Ok(id.trim().parse().unwrap())
}
pub fn parse_input<T: IntoEnumIterator + Sized>(input: &str) -> Option<T> {
    // Try to parse the input as a usize
    let num: usize = input.trim().parse().ok()?;

    // Enumerate over each variant and its index
    for (index, variant) in T::iter().enumerate() {
        // If the input number matches the index, return the variant
        if num == index + 1 {
            return Some(variant);
        }
    }

    // If no match was found, return None
    None
}
