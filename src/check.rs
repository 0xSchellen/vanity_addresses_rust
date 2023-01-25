extern crate redis;
use redis::{Commands};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::H160;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;


pub fn check(task: &str, first: u64, last: u64) {
    println!("1 - Checking  addresses");
    // 0 - Set performance parameters
    let mut now = SystemTime::now();

    // 1 - Set database handlers
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // 2 - Generate and check deterministic addresses   
    for i in first..=last {
        let seed = i as u32;

        // Create seed_arrays
        let (arr_a, arr_b, arr_c, arr_d, arr_e, arr_f, arr_g, arr_h) = create_seed_array(seed);

        // Generate multiple addresses (one for each 32 bits region -> 8 x 32 = 256 bit key)
        let vec_arr: [[u8; 32]; 8] = [arr_a, arr_b, arr_c, arr_d, arr_e, arr_f, arr_g, arr_h];

        // println!("--------------");
        for elem in vec_arr {
            // Generate Signing Key
            let signing_key: SigningKey = SigningKey::from_bytes(&elem).unwrap();
            let signing_key_hex = hex::encode(&elem);

            // Generate Address
            let address: H160 = ethers::utils::secret_key_to_address(&signing_key);
            let mut address_str = hex::encode(&address);
            address_str.insert_str(0, "0x");

            // Test pre-existing address in th database
            // if seed == 4_000_000_009 {
            //     address_str = String::from("0x4569493e319f7041e79dd8a7ab61276550d460b7");
            // }
            // if seed == 4_000_000_010 {
            //    address_str = String::from("0xdf9eb223bafbe5c5271415c75aecd68c21fe3d7f"); 
            // }

            let address = address_str.to_owned();

            // Verify (query) if the address is recorded in the Redis database
            let result = con.get(&address).unwrap_or(0i32);
            // println!("balance {:?} ", result);
    
            match result {
                1 =>  {
                    println!("========== Found =======================================");
                    println!("seed   : {seed:?}");  
                    println!("elem   : {elem:?}"); 
                    println!("address: {address_str:?}");
                    println!("key    : {signing_key_hex:?} ");                       
                    println!("_"); 

                    let mut output = String::new();
                    fmt::write(
                        &mut output,
                        format_args!("--- Found! ---\n {}-{}-{:}\n{:?}\n",
                        seed, address, signing_key_hex, elem),
                    )
                    .expect("Error occurred while trying to write in String");
                    write_results(&task, &output);
                }
                _ => {}
            }

            // println!("{} - {}", seed, address);

            if i % 1_000_000 == 0 {
                let total_time = now.elapsed().unwrap().as_millis() / 1000;

                let mut output = String::new();
                fmt::write(&mut output, format_args!("{i} - time {total_time} s\n"))
                    .expect("Error occurred while trying to write in String");
                write_results(&task, &output);

                println!("{task}: {seed:?}-{elem:?}-{address_str:?}-{signing_key_hex:}");

                // Reset timer
                now = SystemTime::now();
            }
        }
    }
}

pub fn write_results(task: &str, output: &str) {
    let mut path = "results.txt";
    if task == "0" {
        path = "results0.txt";
    }
    if task == "1" {
        path = "results1.txt";
    }
    if task == "2" {
        path = "results2.txt";
    }
    if task == "3" {
        path = "results3.txt";
    }
    if task == "4" {
        path = "results4.txt";
    }
    if task == "5" {
        path = "results5.txt";
    }
    let mut file_ref = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Unable to open file");

    file_ref
        .write_all(output.as_bytes())
        .expect("Atenção: gravação no arquivo result.txt falhou!");
}

fn create_seed_array(
    x: u32,
) -> (
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
) {
    // 64 bytes seed
    // let b1: u8 = ((x >> 56) & 0xff) as u8;
    // let b2: u8 = ((x >> 48) & 0xff) as u8;
    // let b3: u8 = ((x >> 40) & 0xff) as u8;
    // let b4: u8 = ((x >> 32) & 0xff) as u8;
    // let b5: u8 = ((x >> 24) & 0xff) as u8;
    // let b6: u8 = ((x >> 16) & 0xff) as u8;
    // let b7: u8 = ((x >> 8) & 0xff) as u8;
    // let b8: u8 = (x & 0xff) as u8;

    // 32 bytes seed
    // let b1: u8 = ((x >> 56) & 0xff) as u8;
    // let b2: u8 = ((x >> 48) & 0xff) as u8;
    // let b3: u8 = ((x >> 40) & 0xff) as u8;
    // let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;

    let arr_h: [u8; 32] = [
        b4, b3, b2,b1, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_g: [u8; 32] = [
        0, 0, 0, 0, 
        b4, b3, b2,b1, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_f: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        b4, b3, b2,b1, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_e: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        b4, b3, b2,b1, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_d: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        b4, b3, b2,b1, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_c: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        b4, b3, b2,b1, 
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let arr_b: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        b4, b3, b2,b1, 
        0, 0, 0, 0,
    ];

    let arr_a: [u8; 32] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
        b4, b3, b2,b1, 
    ];

    (arr_a, arr_b, arr_c, arr_d, arr_e, arr_f, arr_g, arr_h)
}
