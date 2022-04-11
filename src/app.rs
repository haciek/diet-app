use chrono::prelude::*;
use std::process::exit;
use std::error::Error;

// importing Data struct
use dataframe::Data;


const CSV_PATH: &str = "./csv/weight_data.csv";

pub fn sel_option(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let opt: &str = &args[1];

    match opt {
        "print" => print()?,
        "input" => input(&args[2])?,
        "modify"=> modify(&args[2], &args[3])?,
        _       => exit(1),
    };

    Ok(())
}


pub fn print() -> Result<(), Box<dyn Error>>{
    let records: Vec<Data> = get_records()?;

    for record in records {
        println!("{},\t{},\t{}",
            record.id, record.date, record.weight);
    }
    Ok(())
}


pub fn input(arg_weight: &str) -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Data>  = get_records()?;
    let date: String            = get_date()?;
    let weight: f32             = arg_weight.trim().parse()?;
    let id: u32                 = match records.last() {
        Some(new_id) => new_id.id + 1,
        None => 1,
    };

    let new_record: Data = Data { id, date, weight, };
    records.push(new_record);

    write_records(records)?;
    print()?;
    Ok(())
}


pub fn modify(arg_id: &str, arg_weight: &str) -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Data> = get_records()?;

    for record in records.iter_mut() {
        if record.id == arg_id.parse()? {
            record.weight = arg_weight.parse()?;
        }
    }

    write_records(records)?;
    print()?;
    Ok(())
}


fn write_records(records: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(CSV_PATH)?;

    for record in records {
        wtr.serialize(&record)?;
    }

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
