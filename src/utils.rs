use crate::Currency;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_currency(amount: &str, currency: Currency) -> String {
    match currency {
        Currency::GBP => format!("£{}", amount),
        Currency::EUR => format!("EUR {}", amount),
        Currency::CHF => format!("{} CHF", amount),
        Currency::PLN => format!("{} PLN", amount),
        Currency::LTC => format!("{} LTC", amount),
        Currency::BTC => format!("{} BTC", amount),
    }
}
