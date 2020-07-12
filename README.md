How to run:

Dependencies:
  - cargo

The program expects one argument, which is the name of the csv file to process
```
git clone https://github.com/Arunscape/code4canadachallenge.git
cd code4canadachallenge
cargo run --release C4C-dev-challenge-2018.csv
```

### In case you don't have cargo, but you do have docker
You can compile the program in a docker container

```bash
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.44 cargo build --release
```

The binary will be located at target/release/code4canadachallenge
So, you can run the following command after

```
./target/release/code4canadachallenge C4C-dev-challenge-2018.csv
```

This program calculates:
The number of violations in each category
The earliest and latest violation date for each category

Output should be in a YAML parsable format like this:

```yaml
Biohazards:
    count: 6
    first_date: 2012-04-13 0:00
    last_date: 2012-12-18 0:00
Building Conditions:
    count: 61
    first_date: 2012-01-12 0:00
    last_date: 2012-12-26 0:00
Animals and Pests:
    count: 179
    first_date: 2012-01-03 0:00
    last_date: 2012-12-28 0:00
Chemical Hazards:
    count: 16
    first_date: 2012-02-08 0:00
    last_date: 2012-12-06 0:00
Retail Food:
    count: 0
    first_date: 2012-12-20 0:00
    last_date: 2012-12-20 0:00
Unsanitary Conditions:
    count: 82
    first_date: 2012-01-03 0:00
    last_date: 2012-12-19 0:00
Vegetation:
    count: 66
    first_date: 2012-02-01 0:00
    last_date: 2012-12-05 0:00
Air Pollutants and Odors:
    count: 1
    first_date: 2012-12-05 0:00
    last_date: 2012-12-19 0:00
Garbage and Refuse:
    count: 125
    first_date: 2012-01-03 0:00
    last_date: 2012-12-21 0:00
```

