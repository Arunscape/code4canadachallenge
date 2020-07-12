#![deny(clippy::all)]
use csv;
use serde::Deserialize;
use std::{collections::HashMap, env, fs};

#[derive(Deserialize, Debug)]
struct BuildingCodeViolationRecord {
    violation_id: String,
    inspection_id: String,
    violation_category: String,
    violation_date: String,
    violation_date_closed: String,
    violation_type: String,
}

#[derive(Debug)]
struct ViolationStatistic {
    count: usize,
    first_date: String,
    last_date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let csv_filename = env::args().nth(1).expect("csv file not specified!");

    let file = fs::File::open(csv_filename)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut statistics: HashMap<String, ViolationStatistic> = HashMap::new();

    for record in reader.records() {
        let record = record?;
        let record: BuildingCodeViolationRecord = record.deserialize(None)?;

        statistics
            .entry(record.violation_category.clone())
            .and_modify(|e| {
                e.count += 1;
                if record.violation_date < e.first_date {
                    e.first_date = record.violation_date.clone();
                    return;
                }
                if e.last_date < record.violation_date {
                    e.last_date = record.violation_date.clone();
                    return;
                }
            })
            .or_insert(ViolationStatistic {
                count: 0,
                first_date: record.violation_date.clone(),
                last_date: record.violation_date.clone(),
            });
    }

    for (category, stat) in statistics {
        println!(
            "{}:
    count: {}
    first_date: {}
    last_date: {}",
            category, stat.count, stat.first_date, stat.last_date
        );
    }

    Ok(())
}
