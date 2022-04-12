use crate::dataframe::Data;
use chrono::prelude::*;
use std::error::Error;

const CSV_PATH: &str = "./csv/weight_data.csv";

pub fn print() -> Result<(), Box<dyn Error>>{
    let records: Vec<Data> = get_records()?;

    println!("\tId,\tDate,\tWeight(kg)");
    for record in records {
        println!("\t{},\t{},\t{}",
            record.id, record.date, record.weight);
    }
    Ok(())
}


pub fn input() -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Data>  = get_records()?;

    print()?;
    let date: String = get_date()?;
    let weight: f32  = get_weight()?;
    let id: u32      = match records.last() {
        Some(new_id) => new_id.id + 1,
        None => 1,
    };

    let new_record: Data = Data { id, date, weight, };
    records.push(new_record);

    write_records(records)?;
    print()?;
    Ok(())
}


pub fn modify() -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Data> = get_records()?;

    print()?;
    let id: u32     = get_id()?;
    let weight: f32 = get_weight()?;

    for record in records.iter_mut() {
        if record.id == id {
            record.weight = weight;
        }
    }

    write_records(records)?;
    print()?;
    Ok(())
}


pub fn delete() -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}


fn write_records(records: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(CSV_PATH)?;

    for record in records { wtr.serialize(&record)?; }

    wtr.flush()?;
    Ok(())
}

fn get_records() -> Result<Vec<Data>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(CSV_PATH)?;

    let records: Vec<Data> = rdr
        .deserialize()
        .collect::
        <Result<Vec<Data>, csv::Error>>()?;
    Ok(records)
}

fn get_date() -> Result<String, Box<dyn Error>> {
    let dt   = Utc::today();
    let date = Utc.ymd(dt.year(), dt.month(), dt.day()).format("%d/%m/%Y").to_string();
    Ok(date)
}

fn get_weight() -> Result<f32, Box<dyn Error>> {
    println!("Input weight: ");
    let mut weight = String::new();

    std::io::stdin()
        .read_line(&mut weight)?;

    let weight: f32 = weight.trim().parse::<f32>()?;
    Ok(weight)
}

fn get_id() -> Result<u32, Box<dyn Error>> {
    println!("Select id: ");
    let mut id = String::new();

    std::io::stdin()
        .read_line(&mut id)?;

    let id: u32 = id.trim().parse::<u32>()?;
    Ok(id)
}
