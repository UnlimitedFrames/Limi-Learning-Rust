extern crate csv;
use std::{collections::HashMap, hash::Hash};

use csv::Error;
type Record = HashMap<String,String>;

fn main() -> Result<(), Box<Error>> {
    let mut centeral_points = csv::Reader::from_path("src/cents.csv")?;
    let mut populations = csv::Reader::from_path("src/pop.csv")?;
    let mut popmaps: Vec<Record> = Vec::new();
    let mut centmaps: Vec<Record> = Vec::new();
    
    for result in centeral_points.deserialize() {
        let record: Record = result?;
        centmaps.push(record);
    }
    for result in populations.deserialize() {
        let record: Record = result?;
        popmaps.push(record);
    }


    let mut Combined: Vec<Record> = Vec::new();
    for popdatapoint in &popmaps{
        for centdatapoint in &centmaps{
            if popdatapoint["SA22018_V1_NAME"] == centdatapoint["SA22018_V1_NAME"] {
                let mut raws = HashMap::new();

                raws.insert(String::from("lat"), String::from(&centdatapoint["LATITUDE"]));
                raws.insert(String::from("long"), String::from(&centdatapoint["LONGITUDE"]));
                raws.insert(String::from("2006"), String::from(&popdatapoint["Pop_Total_2006"]));
                raws.insert(String::from("2013"), String::from(&popdatapoint["Pop_Total_2013"]));
                raws.insert(String::from("2018"), String::from(&popdatapoint["Pop_Total_2018"]));

                Combined.push(raws)
            }

        } 
    }
    let mut y2006_pop = 0.0;
    let mut y2013_pop = 0.0;
    let mut y2018_pop = 0.0;
    let mut y2006_raa = 0.0;
    let mut y2013_raa = 0.0;
    let mut y2018_raa = 0.0;
    let mut y2006_roa = 0.0;
    let mut y2013_roa = 0.0;
    let mut y2018_roa = 0.0;
    
    

    for place in Combined{
        y2006_pop = y2006_pop + place["2006"].parse::<f64>().unwrap();
        y2013_pop = y2013_pop + place["2013"].parse::<f64>().unwrap();


        y2018_pop = y2018_pop + place["2018"].parse::<f64>().unwrap();
        y2018_raa = y2018_raa +  (place["2018"].parse::<f64>().unwrap()* place["lat"].parse::<f64>().unwrap() );
        y2018_roa = y2018_roa +  (place["2018"].parse::<f64>().unwrap()* place["long"].parse::<f64>().unwrap() );
        
    }   

    let meaned_lat = y2018_raa/y2018_pop;
    let meaned_long = y2018_roa/y2018_pop;
    println!(" {meaned_lat},{meaned_long}");


    Ok(())
}
