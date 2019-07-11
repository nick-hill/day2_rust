pub fn run() {
    println!("Enter a currency: ");
    let currency : String = read!();
    println!("{}", country(currency));
}

fn country (currency : String)-> String {
    match currency.as_ref() {
        "AUD" => String::from("AUSTRALIA"),
        "CAD" => String::from("CANADA"),
        "INR" => String::from("INDIA"),
        "PEN" => String::from("PERU"),
        "RUB" => String::from("RUSSIA"),
        "USD" => String::from("USA"),
        _ => String::from("NONE"),
    }
}