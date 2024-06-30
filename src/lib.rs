use core::panic;
use std::io::BufReader;
use std::fs::File;
use std::fmt;
use chrono::NaiveDate;

mod utils;
mod tokens;
use crate::tokens::*;
use crate::utils::*;


#[derive(Debug, Copy, Clone)]
pub enum Currency {
    GBP,
    EUR,
    CHF,
    PLN,
    LTC,
    BTC,
}

impl Currency {
    pub fn from_str(currency: &str) -> Currency {
        match currency.to_lowercase().as_str() {
            "gbp" => Currency::GBP,
            "eur" => Currency::EUR,
            "chf" => Currency::CHF,
            "pln" => Currency::PLN,
            "ltc" => Currency::LTC,
            "btc" => Currency::BTC,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatementType {
    ClCardCSV,
    ChaseCSV,
    Unknown,
}

#[derive(Debug)]
pub struct BankStatement {
    transactions: Vec<LedgerEntry>,
}

impl BankStatement {
    pub fn from_csv(path: &str, statement_type: StatementType) -> Option<BankStatement> {
        let f = File::open(path).ok()?;
        let reader = BufReader::new(f);
        let mut csv_reader = csv::Reader::from_reader(reader);

        let transactions: Vec<LedgerEntry> = csv_reader.records().map(|result| {
            let record = result.unwrap();
            LedgerEntry::from_record(record, statement_type)
        }).collect();

        Some(BankStatement {
            transactions,
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

#[derive(Debug, Copy, Clone)]
pub enum TransactionType {
    Transfer,
    Purchase,
    Payment,
    Refund,
    Interest
}

impl TransactionType {
    pub fn from_str(transaction_type: &str) -> TransactionType {
        match transaction_type.to_lowercase().as_str() {
            "transfer" => TransactionType::Transfer,
            "purchase" => TransactionType::Purchase,
            "payment" => TransactionType::Payment,
            "refund" => TransactionType::Refund,
            "interest" => TransactionType::Interest,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct LedgerEntry {
    date: String,
    description: String,
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
                let date = NaiveDate::parse_from_str(card_row.timestamp, DATE_FMT).unwrap();
                let description = card_row.get_description();

                LedgerEntry {
                    date: date.format(DATE_FMT).to_string(),
                    description: description.clone(),
                    destination: description_to_account(&description),
                    destination_amount: card_row.source.to_string(),
                    source: CL_LTC.to_string(),
                    source_amount: "".to_string(),
                }
            },
            StatementType::ChaseCSV => {
                let card_row: ChaseRow = record.deserialize(None).unwrap();
                let transaction_type = TransactionType::from_str(card_row.transaction_type);
                let description = card_row.get_description();
                let date = NaiveDate::parse_from_str(card_row.date, "%d %b %Y").unwrap();

                match transaction_type {
                    TransactionType::Purchase | TransactionType::Transfer
                    => LedgerEntry {
                        date: date.format(DATE_FMT).to_string(),
                        description: description.clone(),
                        destination: description_to_account(&description),
                        destination_amount: format_currency(
                            &card_row.amount[1..], Currency::from_str(card_row.currency)
                        ),
                        source: CHASE.to_string(),
                        source_amount: "".to_string(),
                    },

                    TransactionType::Payment | TransactionType::Refund | TransactionType::Interest
                    => LedgerEntry {
                        date: date.format(DATE_FMT).to_string(),
                        description: description.clone(),
                        destination: CHASE.to_string(),
                        destination_amount: format_currency(
                            card_row.amount, Currency::from_str(card_row.currency)
                        ),
                        source: description_to_account(&description),
                        source_amount: "".to_string(),
                    },
                }
            }
            StatementType::Unknown => {
                panic!();
            }
        }
    }
}

impl fmt::Display for LedgerEntry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let source_space = "";
        let source_amount = self.source_amount.to_uppercase();

        let destination_space = " ".repeat(40 - 4 - self.destination.len());
        let destination_amount = self.destination_amount.to_uppercase();

        let result = format!("{date} * {name}
        {destination}{d_space}{d_amount}
        {source}{s_space}{s_amount}",
            date=self.date, name=self.description,
            destination=self.destination, d_space=destination_space, d_amount=destination_amount,
            source=self.source, s_space=source_space, s_amount=source_amount,
        );

        fmt.write_str(&result)?;
        Ok(())
    }
}

// *** CSV Deserialize Row Structs ***

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

impl<'a> CLCardRow<'a> {
    pub fn get_description(&self) -> String {
        let mut name_parts: Vec<String> = self.merchant.split(' ').filter(
            |&x| !x.is_empty()
        ).map(|x| capitalize(&x.to_lowercase())).collect();
        name_parts.pop();
        name_parts.pop();

        let mut name = name_parts.join(" ");

        if name.starts_with("Crv*") {
            name = capitalize(&name[4..name.len()-7]);
        }

        match name.as_str() {
            "Mctuckys" => "McTucky's",
            "Trainpal" => "TrainPal",
            "Hellofresh" => "HelloFresh",
            "Creditladder" => "CreditLadder",
            "Transport For Greater Manchester" => "Transport for Greater Manchester",
            _ => &name,
        }.to_string()
    }
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct ChaseRow<'a> {
    date: &'a str,
    time: &'a str,
    transaction_type: &'a str,
    description: &'a str,
    amount: &'a str,
    currency: &'a str,
    balance: &'a str,
}

impl<'a> ChaseRow<'a> {
    pub fn get_description(&self) -> String {
        match self.description {
            "Miu - SumUp" => "MIU",
            "Sikorski Memorial Hous" => "Sikorski Memorial House",
            _ => self.description
        }.to_string()
    }
}


