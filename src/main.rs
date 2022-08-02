extern crate csv;
use csv::{Error, StringRecord};


fn main() -> Result<(), Box<Error>>{
	let mut centeral_points = csv::Reader::from_path("src/cents.csv")?;
	let mut populations =csv::Reader::from_path("src/pop.csv")?; 
	for result in centeral_points.records() {
		let record = result?;
        println!("{:?}", record);
	}
	
    Ok(())
}