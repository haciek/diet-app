use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use std::{
	collections::HashMap,
	env,
	error::Error,
	fs::{create_dir, File},
	path::Path,
};

use crate::dataframe::Record;

const OS_DIR: &str = {
	match option_env!("XDG_CONFIG_HOME") {
		None => env!("HOME"),
		Some(dir) => dir,
	}
};
const CSV_DIR_NAME: &str = "wtcli";
const CSV_FILE_NAME: &str = "weight-data.csv";

pub fn display() -> Result<(), Box<dyn Error>> {
	summary()?;
	records()?;
	Ok(())
}

pub fn records() -> Result<(), Box<dyn Error>> {
	let records: &Vec<Record> = &get_records()?;

	println!("\n\tId\tDate\t\tWeek\tWeight(kg)");
	for record in records.iter() {
		println!(
			"\t{}\t{}\t{}\t{}",
			&record.id, &record.date, &record.week, &record.weight
		);
	}

	Ok(())
}

pub fn summary() -> Result<(), Box<dyn Error>> {
	let mut avg_w_weight = HashMap::<i64, f32>::new();
	let mut avg_w_count = HashMap::<i64, i32>::new();
	let mut avg_weight = 0.0;

	let records: &Vec<Record> = &get_records()?;
	for record in records.iter() {
		avg_weight += &record.weight;
		// gets weight for each week
		avg_w_weight.insert(
			record.week,
			*avg_w_weight.get(&record.week).unwrap_or(&0.0) + record.weight,
		);
		// gets the number of recorded weights per week
		avg_w_count.insert(
			record.week,
			*avg_w_count.get(&record.week).unwrap_or(&0) + 1,
		);
	}
	// averages the weight of each week
	for (key, val) in avg_w_weight.iter_mut() {
		*val /= *avg_w_count.get(&key).unwrap_or(&0) as f32
	}

	let mut sorted_week_weight: Vec<(&i64, &f32)> = avg_w_weight.iter().collect();
	sorted_week_weight.sort_by_key(|key| key.0);

	avg_weight /= records.len() as f32;
	println!("\n\tAverage overall weight:\n\t{}", avg_weight);
	println!("\n\tAverage weight per week:\n\tWeek\tWeight");

	for (key, val) in sorted_week_weight.iter_mut() {
		println!("\t{}\t{}", key, val);
	}

	Ok(())
}

pub fn input(weight: f32) -> Result<(), Box<dyn Error>> {
	let mut records: Vec<Record> = get_records()?;
	let date = get_date()?.to_string();

	let mut week = 1;
	if !records.is_empty() {
		let last_date = &records.last().unwrap().date;
		let last_week = &records.last().unwrap().week;
		week = get_week(*last_week, &last_date, &date)?;
		println!("{} {}", last_date, last_week);
	}

	let id: u32 = match records.last() {
		Some(new_id) => new_id.id + 1,
		None => 1,
	};

	let new_record = Record {
		id,
		date,
		week,
		weight,
	};
	records.push(new_record);

	write_records(&records)?;
	display()?;
	Ok(())
}

pub fn modify(id: u32, weight: f32) -> Result<(), Box<dyn Error>> {
	let mut records: Vec<Record> = get_records()?;

	for record in records.iter_mut() {
		if record.id == id {
			record.weight = weight;
		}
	}

	write_records(&records)?;
	display()?;
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
	write_records(&records)?;
	display()?;
	Ok(())
}

fn write_records(records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
	let csv_path = get_valid_path()?;
	let mut wtr = csv::Writer::from_path(&csv_path)?;
	for record in records {
		wtr.serialize(&record)?;
	}
	wtr.flush()?;
	Ok(())
}

fn get_records() -> Result<Vec<Record>, Box<dyn Error>> {
	let csv_path = &get_valid_path()?;
	let mut rdr = csv::Reader::from_path(csv_path)?;
	let records: Vec<Record> = rdr
		.deserialize()
		.collect::<Result<Vec<Record>, csv::Error>>()?;
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

fn get_week(prev_week: i64, prev_date: &str, curr_date: &str) -> Result<i64, Box<dyn Error>> {
	let prev_date = NaiveDate::parse_from_str(&prev_date, "%d/%m/%Y")?;
	let curr_date = NaiveDate::parse_from_str(&curr_date, "%d/%m/%Y")?;

	let weeks = (curr_date - prev_date).num_weeks();

	let curr_week = prev_week + weeks;
	Ok(curr_week)
}

fn get_valid_path() -> Result<String, std::io::Error> {
	let csv_dir = format!("{}/{}/", OS_DIR, CSV_DIR_NAME);
	let csv_path = format!("{}{}", csv_dir, CSV_FILE_NAME);

	if !Path::new(&csv_dir).exists() {
		create_dir(&csv_dir)?;
	}
	if !Path::new(&csv_path).exists() {
		File::create(&csv_path)?;
	}

	Ok(csv_path)
}
