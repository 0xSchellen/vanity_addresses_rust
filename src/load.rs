use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;
extern crate redis;
use redis::{Commands};

pub fn load() {
    // 0 - Set performance parameters
    let now = SystemTime::now();
    let mut line_count: i32 = 0;

    // 1 - Set database handlers
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // 2 - Set csv input file handler
    println!("1 - Loading addresses file");

    let file = File::open("eth_addresses.csv").expect("Problem opening the file eth_addresses.csv");
    let reader = BufReader::new(file);

    // Read input eth_addresses.csv file line by line
    for line in reader.lines() {
        line_count = line_count + 1;

        let line = line.unwrap();
        let address = line.to_owned();

        // Insert address record in the database
        // const RV: Result<_, RedisError> = ;
        let _result : () =  con.set(&address, 1u32).unwrap();

        // println!("{:?} - {} - {} --> Inserted!", result, line_count, address);

        let _result = con.get(&address).unwrap_or(0i32);
        // println!("balance {:?} ", result);

        // match result {
        //     1 => println!("{:?} - {} - {} --> Found!", result, line_count, address),
        //     0 => println!("{:?} - {} - {} --> Not Found!", result, line_count, address),
        //     _ => {}
        // }
        println!("{} - {}", line_count, address);
    }
    let total_time = now.elapsed().unwrap();
    println!(
        "Total addresses: {} - Time to load data: {} ",
        line_count,
        total_time.as_millis() / 1000
    );
}
