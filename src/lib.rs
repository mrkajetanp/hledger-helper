use std::io::BufReader;
use std::fs::File;

static GROCERIES: &str = "Expenses:Groceries";
static ALCOHOL: &str = "Expenses:Alcohol";
static EATING_OUT: &str = "Expenses:Eating Out";

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct CLCardRow<'a> {
    timestamp: &'a str,
    merchant: &'a str,
    merchant_type: &'a str,
    transaction_currency: &'a str,
    transaction_amount: f64,
    card_currency: &'a str,
    card_amount: f64,
    ma_exchange_rate: &'a str,
    ecb_exchange_rate: &'a str,
    markup: &'a str,
    source: &'a str,
}

fn name_to_destination(name: &str) -> String {
    match name {
        "Iceland" |
        "Aldi" |
        "Ryman"
        => GROCERIES,

        "McTucky's"
        => EATING_OUT,

        "On Bar Manchester" |
        "Brewdog Man Doghouse"
        => ALCOHOL,

        _ => "Expenses:",
     }.to_string()
}

fn process_name(name: &str) -> String {
    let mut name_parts: Vec<String> = name.split(' ').filter(|&x| !x.is_empty())
        .map(|x| capitalize(&x.to_lowercase())).collect();
    name_parts.pop();
    name_parts.pop();

    let mut name = name_parts.join(" ");

    if name.starts_with("Crv*") {
        name = capitalize(&name[4..name.len()-7]);
    }

    match name.as_str() {
        "Mctuckys"
        => "McTucky's",

        _ => &name,
    }.to_string()
}

fn ledger_entry_simple(date: &str, name: &str, destination: &str, destination_amount: &str, source: &str, source_amount: &str) -> String {
    let source_space = "";
    let destination = if destination.is_empty() { &name_to_destination(name) } else { destination };
    let destination_amount = destination_amount.to_uppercase();
    let source_amount = source_amount.to_uppercase();
    let destination_space = " ".repeat(40 - 4 - destination.len());

    format!("{date} * {name}
    {destination}{d_space}{d_amount}
    {source}{s_space}{s_amount}",
        date=date, name=name,
        destination=destination, d_space=destination_space, d_amount=destination_amount,
        source=source, s_space=source_space, s_amount=source_amount,
    )
}

pub fn from_csv(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut csv_reader = csv::Reader::from_reader(reader);

    for result in csv_reader.records() {
        let record = result?;
        let card_row: CLCardRow = record.deserialize(None)?;

        let entry = ledger_entry_simple(card_row.timestamp, &process_name(card_row.merchant),
            "", card_row.source, "Assets:Crypto:CL:LTC", ""
        );

        println!("{}\n", entry);
    }

    Ok(())
}
