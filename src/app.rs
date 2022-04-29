use crate::dataframe::Data;
use chrono::prelude::*;
use std::error::Error;

const CSV_PATH: &str = "./csv/weight_data.csv";

pub fn print() -> Result<(), Box<dyn Error>> {
	let records: Vec<Data> = get_records()?;

	println!("\tId,\tDate,\t\tWeight(kg)");
	for record in records {
		println!("\t{},\t{},\t{}", record.id, record.date, record.weight);
	}
	Ok(())
}

pub fn input(weight: f32) -> Result<(), Box<dyn Error>> {
	let mut records: Vec<Data> = get_records()?;

	let date: String = get_date()?;
	let id: u32 = match records.last() {
		Some(new_id) => new_id.id + 1,
		None => 1,
	};

	let new_record: Data = Data { id, date, weight };
	records.push(new_record);

	write_records(records)?;
	print()?;
	Ok(())
}

pub fn modify(id: u32, weight: f32) -> Result<(), Box<dyn Error>> {
	let mut records: Vec<Data> = get_records()?;

	for record in records.iter_mut() {
		if record.id == id {
			record.weight = weight;
		}
	}

	write_records(records)?;
	print()?;
	Ok(())
}

pub fn delete(id: u32) -> Result<(), Box<dyn Error>> {
	let mut records = get_records()?;
	let i = id - 1;

	records.remove(i.try_into().unwrap());
	for (i, record) in records.iter_mut().enumerate() {
		let i: u32 = (i + 1).try_into()?;
		record.id = i;
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
		.collect::<Result<Vec<Data>, csv::Error>>()?;
	Ok(records)
}

fn get_date() -> Result<String, Box<dyn Error>> {
	let dt = Utc::today();
	let date = Utc
		.ymd(dt.year(), dt.month(), dt.day())
		.format("%d/%m/%Y")
		.to_string();
	Ok(date)
}

