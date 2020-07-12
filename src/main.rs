use csv;
use csv::StringRecord;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

const CSV_FILENAME: &'static str = "../C4C-dev-challenge-2018.csv";

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
    let file = fs::File::open(CSV_FILENAME)?;
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

    println!("{:?}", statistics);

    Ok(())
}
