use crate::serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use std::{
	fs::{create_dir, File},
	collections::HashMap,
	error::Error,
	path::Path,
	env,
};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
	id: u32,
	date: String,
	week: i64,
	weight: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CsvData {
	filepath: String,
	records: Vec<Record>,
}

impl CsvData {
	pub fn new() -> Result<CsvData, Box<dyn Error>> {
		// checking if csv filepath is valid
		let os_dir = {
			match option_env!("XDG_CONFIG_HOME") {
				None => env!("HOME"),
				Some(dir) => dir,
			}
		};
		let csv_dir_name = "wtcli";
		let csv_file_name = "weight-data.csv";

		let filedir = format!("{}/{}/", &os_dir, &csv_dir_name);
		let filepath = format!("{}{}", &filedir, &csv_file_name);

		if !Path::new(&filedir).exists() { create_dir(&filedir)?; }
		if !Path::new(&filepath).exists() { File::create(&filepath)?; }

		// reads records from the csv file
		let mut rdr = csv::Reader::from_path(&filepath)?;
		let records: Vec<Record> = rdr
			.deserialize()
			.collect::<Result<Vec<Record>, csv::Error>>()?;

		Ok( CsvData { filepath, records, } )
	}

	pub fn append(&mut self, weight: f32) -> Result<(), Box<dyn Error>> {
		// defaults if there are no records
		let mut id: u32 = 1;
		let mut week: i64 = 1;

		let today = Utc::today();
		let date = Utc
			.ymd(today.year(), today.month(), today.day())
			.format("%d/%m/%Y")
			.to_string();

		if let Some(last_record) = self.records.last() {
			id = last_record.id + 1;

			let last_date = NaiveDate::parse_from_str(&last_record.date, "%d/%m/%Y")?;
			let date = NaiveDate::parse_from_str(&date, "%d/%m/%Y")?;
			week = last_record.week + (date - last_date).num_weeks();
		};

		let record = Record { id, date, week, weight };
		self.records.push(record);
		CsvData::save_data(&self)?;
		Ok(())
	}

	pub fn modify(&mut self, id: u32, weight: f32) -> Result<(), Box<dyn Error>> {
		if let Some(record) = self.records.get_mut(id as usize - 1) {
			record.weight = weight;
		};
		self.save_data()?;
		Ok(())
	}

	pub fn delete(&mut self, id: u32) -> Result<(), Box<dyn Error>> {
		self.records.remove(id as usize - 1);
		// fixing id numbers
		for (i, record) in self.records.iter_mut().enumerate() {
			record.id = i as u32 + 1;
		}
		self.save_data()?;
		Ok(())
	}

	pub fn show_all(&self) {
		self.show_data();
		self.show_summary();
	}

	pub fn show_data(&self) {
		println!("\n\tData..");
		println!("\n\tId\tDate\t\tWeek\tWeight(kg)");
		self.records
			.iter()
			.for_each(|r|
				println!("\t{}\t{}\t{}\t{}", r.id, r.date, r.week, r.weight)
		);
	}

	pub fn show_summary(&self) {
		println!("\n\tSummary..");
		let mut weight: f32 = 0.0;
		let mut weight_week = HashMap::<i64, f32>::new();
		let mut count_week = HashMap::<i64, i32>::new();
		self.records
			.iter()
			.for_each(|r| {
				weight += &r.weight;

			weight_week.insert(
				r.week,
				*weight_week.get(&r.week).unwrap_or(&0.0) + r.weight,
			);
			// gets the number of recorded weights per week
			count_week.insert(
				r.week,
				*count_week.get(&r.week).unwrap_or(&0) + 1,
			);

		});

		// averages the weight of each week
		weight_week
			.iter_mut()
			.for_each(|(k, v)|
				*v /= *count_week.get(&k).unwrap_or(&0) as f32
			);

		// sorting weights ascending by week
		let mut weight_week = weight_week
			.iter()
			.collect::<Vec<(&i64, &f32)>>();
		weight_week.sort_by_key(|key| key.0);

		println!("\tAverage weight: {}", weight / self.records.len() as f32);
		println!("\tAverage weight per week:");
		println!("\tWeek\tWeight");
		weight_week
			.iter()
			.for_each(|(k, v)|
				println!("\t{}\t{}", k, v)
		);
	}

	fn save_data(&self) -> Result<(), csv::Error> {
		let mut writer = csv::Writer::from_path(&self.filepath)?;
		for record in &self.records { writer.serialize(record)?; }
		writer.flush()?;
		Ok(())
	}

}

