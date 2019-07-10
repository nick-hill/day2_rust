pub fn run() {
    let currency : String = read!();
    println!("{}", country(currency));
}

fn country (currency : String)-> String {
    match currency {
        AUD => String::from("AUSTRALIA"),
        CAD => String::from("CANADA"),
        INR => String::from("INDIA"),
        PEN => String::from("PERU"),
        RUB=> String::from("RUSSIA"),
        USB => String::from("USA"),
        _ => String::from("NONE"),
    }
}