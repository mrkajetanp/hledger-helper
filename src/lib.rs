use core::panic;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

mod utils;
mod tokens;
use crate::tokens::*;

#[derive(Copy, Clone)]
pub enum StatementType {
    ClCardCSV,
    Unknown,
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
            },
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

        let destination = if self.destination.is_empty() {
            name_to_destination(&self.name)
        } else {
            self.destination.clone()
        };
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
