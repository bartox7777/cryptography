use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
};

use digest::Digest;

use rand::Rng;

fn generate_benchmark_file(size_mb: usize) -> String {
    let file_name = format!("{}MB.bin", size_mb);
    let mut file = File::create(file_name.as_str()).unwrap();
    let mut rng = rand::thread_rng();
    let mut buffer = [0u8; 1024];
    for _ in 0..size_mb {
        for _ in 0..1024 {
            rng.fill(&mut buffer);
            file.write_all(&buffer).unwrap();
        }
    }
    file.sync_all().unwrap();
    file_name
}

fn generate_hash_from_file(filename: &String, mut hasher: impl Digest) -> Vec<u8> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 1024];

    let start = std::time::Instant::now(); // Start timer

    loop {
        let n = reader.read(&mut buffer);
        // try unwrap n
        let n = match n {
            Ok(n) => n,
            Err(e) => {
                panic!("Error reading file: {:?}", e);
            }
        };
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize().to_vec();

    let elapsed = start.elapsed(); // Stop timer
                                   // println!(
                                   //     "  Elapsed: {:?}. Hash: {:x?} (size: {}b)",
                                   //     elapsed,
                                   //     hash,
                                   //     hash.len() * 8
                                   // ); // Print time taken

    hash
}

fn main() {
    /* ZAD 2 */
    // let one_mb = generate_benchmark_file(1);
    // let five_mb = generate_benchmark_file(5);
    // let ten_mb = generate_benchmark_file(10);

    // println!("MD5:");
    // generate_hash_from_file(&one_mb, md5::Md5::new());
    // generate_hash_from_file(&five_mb, md5::Md5::new());
    // generate_hash_from_file(&ten_mb, md5::Md5::new());

    // println!("SHA1:");
    // generate_hash_from_file(&one_mb, sha1::Sha1::new());
    // generate_hash_from_file(&five_mb, sha1::Sha1::new());
    // generate_hash_from_file(&ten_mb, sha1::Sha1::new());

    // println!("SHA256:");
    // generate_hash_from_file(&one_mb, sha2::Sha256::new());
    // generate_hash_from_file(&five_mb, sha2::Sha256::new());
    // generate_hash_from_file(&ten_mb, sha2::Sha256::new());

    // println!("SHA3-256:");
    // generate_hash_from_file(&one_mb, sha3::Sha3_256::new());
    // generate_hash_from_file(&five_mb, sha3::Sha3_256::new());
    // generate_hash_from_file(&ten_mb, sha3::Sha3_256::new());

    /* ZAD 3 */
    // let hash = generate_hash_from_file(&String::from("1234.txt"), md5::Md5::new());
    // for byte in hash {
    //     if byte == 0 {
    //         print!("00");
    //     } else {
    //         print!("{:x}", byte);
    //     }
    // }
    // println!();

    /* ZAD 5 */
    // let mut first_12bits: HashMap<u16, u8> = HashMap::new();
    // for _ in 1..=1000 {
    //     let one_mb = generate_benchmark_file(1);
    //     let hash = generate_hash_from_file(&one_mb, md5::Md5::new());
    //     let bits = hash[0] as u16 + (((hash[1] & 0b00001111) as u16) << 8);
    //     first_12bits
    //         .entry(bits)
    //         .and_modify(|e| *e += 1)
    //         .or_insert(1);
    // }

    // let mut max = 0;
    // for value in first_12bits.values() {
    //     if *value > max {
    //         max = *value;
    //     }
    // }
    // println!("Max value: {}", max);

    /* ZAD 6 */
    let one_mb = generate_benchmark_file(1);

    let hash = generate_hash_from_file(&one_mb, md5::Md5::new());

    // modify one bit in file
    let mut file = File::open(one_mb).unwrap();
    let mut buffer = [0u8; 1024];
    file.read(&mut buffer).unwrap();
    buffer[0] = buffer[0] ^ 0b00000001;
    let mut file = File::create("1MB_modified.bin").unwrap();
    file.write_all(&buffer).unwrap();
    file.sync_all().unwrap();

    let hash_modified = generate_hash_from_file(&String::from("1MB_modified.bin"), md5::Md5::new());

    // compare every bit
    let mut diff = 0;
    for i in 0..hash.len() {
        let mut xor = hash[i] ^ hash_modified[i];
        while xor > 0 {
            if xor & 1 == 1 {
                diff += 1;
            }
            xor >>= 1;
        }
    }
    println!(
        "Probability of change: {}",
        diff as f64 / (hash.len() * 8) as f64
    );
}
