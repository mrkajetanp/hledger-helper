use std::io::BufReader;
use std::fs::File;
use std::fmt;

static GROCERIES: &str = "Expenses:Groceries";
static ALCOHOL: &str = "Expenses:Alcohol";
static EATING_OUT: &str = "Expenses:Eating Out";

#[derive(Copy, Clone)]
pub enum StatementType {
    ClCardCSV,
}

#[derive(Debug)]
pub struct BankStatement {
    transactions: Vec<LedgerEntry>,
}

impl BankStatement {
    pub fn from_csv(path: &str, statement_type: StatementType) -> Option<BankStatement> {
        let f = File::open(path).ok()?;
        let mut csv_reader = csv::Reader::from_reader(BufReader::new(f));

        let transactions: Vec<LedgerEntry> = csv_reader.records().map(|result| {
            let record = result.unwrap();
            LedgerEntry::from_record(record, statement_type)
        }).collect();

        Some(BankStatement {
            transactions: transactions,
        })
    }
}

impl fmt::Display for BankStatement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for st in &self.transactions {
            fmt.write_str(&st.to_string())?;
            fmt.write_str("\n\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct LedgerEntry {
    date: String,
    name: String,
    destination: String,
    destination_amount: String,
    source: String,
    source_amount: String,
}

impl LedgerEntry {
    pub fn from_record(record: csv::StringRecord, statement_type: StatementType) -> LedgerEntry {
        match statement_type {
            StatementType::ClCardCSV => {
                let card_row: CLCardRow = record.deserialize(None).unwrap();
                LedgerEntry {
                    date: card_row.timestamp.to_string(),
                    name: process_name(card_row.merchant),
                    destination: "".to_string(),
                    destination_amount: card_row.source.to_string(),
                    source: "Assets:Crypto:CL:LTC".to_string(),
                    source_amount: "".to_string(),
                }
            }
        }
    }
}

impl fmt::Display for LedgerEntry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let source_space = "";
        let source_amount = self.source_amount.to_uppercase();

        let destination = if self.destination.is_empty() { name_to_destination(&self.name) } else { self.destination.clone() };
        let destination_space = " ".repeat(40 - 4 - destination.len());
        let destination_amount = self.destination_amount.to_uppercase();

        let result = format!("{date} * {name}
        {destination}{d_space}{d_amount}
        {source}{s_space}{s_amount}",
            date=self.date, name=self.name,
            destination=destination, d_space=destination_space, d_amount=destination_amount,
            source=self.source, s_space=source_space, s_amount=source_amount,
        );

        fmt.write_str(&result)?;
        Ok(())
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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
