use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
};

use digest::Digest;

use rand::Rng;

fn generate_benchmark_file(size_mb: usize) -> String {
    let file_name = format!("{}MB.bin", size_mb); // File name
    let mut file = File::create(file_name.as_str()).unwrap(); // Create file
    let mut rng = rand::thread_rng(); // Random number generator
    let mut buffer = [0u8; 1024]; // Buffer
    for _ in 0..size_mb {
        for _ in 0..1024 {
            rng.fill(&mut buffer); // Fill buffer with random data
            file.write_all(&buffer).unwrap(); // Write buffer to file
        }
    }
    file.sync_all().unwrap(); // Sync file
    file_name // Return file name
}

fn generate_hash_from_file(filename: &String, mut hasher: impl Digest) -> Vec<u8> {
    let file = File::open(filename).unwrap(); // Open file
    let mut reader = BufReader::new(file); // Create reader
    let mut buffer = [0u8; 1024]; // Buffer

    let start = std::time::Instant::now(); // Start timer

    loop {
        let n = reader.read(&mut buffer); // Read from file
                                          // try unwrap n
        let n = match n {
            Ok(n) => n, // If Ok, assign size of read data to n
            Err(e) => {
                panic!("Error reading file: {:?}", e);
            }
        };
        if n == 0 {
            break; // If no data was read, break
        }
        hasher.update(&buffer[..n]); // Update hasher with buffer
    }
    let hash = hasher.finalize().to_vec(); // Finalize hasher and convert to vector

    let elapsed = start.elapsed(); // Stop timer
    println!(
        "  Elapsed: {:?}. Hash: {:x?} (size: {}b)",
        elapsed,
        hash,
        hash.len() * 8
    ); // Print elapsed time and hash

    hash
}

fn main() {
    /* ZAD 2 */
    // // let one_mb = generate_benchmark_file(1);
    // // let five_mb = generate_benchmark_file(5);
    // // let ten_mb = generate_benchmark_file(10);

    // // println!("MD5:");
    // // generate_hash_from_file(&one_mb, md5::Md5::new());
    // // generate_hash_from_file(&five_mb, md5::Md5::new());
    // // generate_hash_from_file(&ten_mb, md5::Md5::new());

    // // println!("SHA1:");
    // // generate_hash_from_file(&one_mb, sha1::Sha1::new());
    // // generate_hash_from_file(&five_mb, sha1::Sha1::new());
    // // generate_hash_from_file(&ten_mb, sha1::Sha1::new());

    // // println!("SHA256:");
    // // generate_hash_from_file(&one_mb, sha2::Sha256::new());
    // // generate_hash_from_file(&five_mb, sha2::Sha256::new());
    // // generate_hash_from_file(&ten_mb, sha2::Sha256::new());

    // // println!("SHA3-256:");
    // // generate_hash_from_file(&one_mb, sha3::Sha3_256::new());
    // // generate_hash_from_file(&five_mb, sha3::Sha3_256::new());
    // // generate_hash_from_file(&ten_mb, sha3::Sha3_256::new());

    // /* ZAD 3 */
    // // let hash = generate_hash_from_file(&String::from("1234.txt"), md5::Md5::new());
    // // for byte in hash {
    // //     if byte == 0 {
    // //         print!("00");
    // //     } else {
    // //         print!("{:x}", byte);
    // //     }
    // // }
    // // println!();

    /* ZAD 5 */
    let mut first_12bits: HashMap<u16, u8> = HashMap::new(); // Dictionary for first 12 bits of each hash
    for _ in 1..=1000 {
        let one_mb = generate_benchmark_file(1); // Generate 1MiB file
        let hash = generate_hash_from_file(&one_mb, sha3::Sha3_256::new()); // Generate hash from file
        let bits = hash[0] as u16 + (((hash[1] & 0b00001111) as u16) << 8); // Get first 12 bits of hash
        first_12bits
            .entry(bits)
            .and_modify(|e| *e += 1)
            .or_insert(1); // Increment value of specific 12 bits in dictionary
    }

    let mut max = 0;
    for value in first_12bits.values() {
        // Find max value
        if *value > max {
            max = *value;
        }
    }
    println!("Max value: {}", max);
    println!("{:x?}", first_12bits);

    /* ZAD 6 */
    let mut avg = 0.0; // Average

    for _ in 1..=1000 {
        let one_mb = generate_benchmark_file(1); // Generate 1MiB file

        let hash = generate_hash_from_file(&one_mb, md5::Md5::new()); // Generate hash from file

        // modify one bit in file
        let mut file = File::open(one_mb).unwrap(); // Open file
        let mut buffer = [0u8; 1024]; // Buffer
        file.read(&mut buffer).unwrap(); // Read file to buffer
        buffer[0] = buffer[0] ^ 0b00000001; // Modify one bit
        let mut file = File::create("1MB_modified.bin").unwrap(); // Create new file
        file.write_all(&buffer).unwrap(); // Write buffer to file
        file.sync_all().unwrap(); // Sync file

        let hash_modified =
            generate_hash_from_file(&String::from("1MB_modified.bin"), md5::Md5::new()); // Generate hash from modified file

        // compare every bit
        let mut diff = 0; // Difference
        for i in 0..hash.len() {
            // For every byte in hash
            let mut xor = hash[i] ^ hash_modified[i]; // XOR hash and modified hash
            while xor > 0 {
                // While there are bits to compare
                if xor & 1 == 1 {
                    // If bits are different
                    diff += 1; // Increment difference
                }
                xor >>= 1; // Shift bits
            }
        }
        avg += diff as f64 / (hash.len() * 8) as f64; // Add difference to avg
    }
    avg /= 1000.0; // Divide avg by number of iterations
    println!("Probability of change: {}", avg);
}
