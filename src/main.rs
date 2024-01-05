use std::error::Error;
use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Feld 1")]
    name_ff: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Umsatz {
    #[serde(rename = "Bezeichnung Auftragskonto")]
    description: String,
    #[serde(rename = "IBAN Auftragskonto")]
    sender_iban: String,
    #[serde(rename = "BIC Auftragskonto")]
    sender_bic: String,
    #[serde(rename = "Bankname Auftragskonto")]
    sender_name: String,
    #[serde(rename = "Buchungstag")]
    booking_day: String,
    #[serde(rename = "Valutadatum")]
    value_date: String,
    #[serde(rename = "Name Zahlungsbeteiligter")]
    recipient_name: Option<String>,
    #[serde(rename = "IBAN Zahlungsbeteiligter")]
    recipient_iban: Option<String>,
    #[serde(rename = "BIC (SWIFT-Code) Zahlungsbeteiligter")]
    recipient_bic: Option<String>,
    #[serde(rename = "Buchungstext")]
    text: String,
    #[serde(rename = "Verwendungszweck")]
    name_ff: String,
    #[serde(rename = "Betrag")]
    amount: String,
    #[serde(rename = "Waehrung")]
    currency: String,
    #[serde(rename = "Saldo nach Buchung")]
    saldo: String,
    #[serde(rename = "Bemerkung")]
    bemerkung: Option<String>,
    #[serde(rename = "Kategorie")]
    category: String,
    #[serde(rename = "Steuerrelevant")]
    tax_relevant: Option<String>,
    #[serde(rename = "Glaeubiger ID")]
    id: Option<String>,
    #[serde(rename = "Mandatsreferenz")]
    reference: Option<String>,
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: Umsatz = result?;
        println!("{:?}", record.text);
    }

    Ok(())
}