use std::{
    fs::File,
    io::{Read, Write},
};

use openssl::{
    symm::Cipher,
    symm::{decrypt, encrypt},
};
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

fn encrypt_file(filename: &String, key: &String, iv: &String, cipher: Cipher) -> String {
    let mut file = File::open(filename).unwrap(); // Open file
    let mut data = Vec::new(); // Vector to store data
    file.read_to_end(&mut data).unwrap(); // Read file to vector
    let encrypted = encrypt(cipher, key.as_bytes(), Some(iv.as_bytes()), &data).unwrap();
    let encrypted_filename = format!("enc_{}", filename); // Encrypted file name
    let mut encrypted_file = File::create(encrypted_filename.as_str()).unwrap();
    encrypted_file.write_all(&encrypted).unwrap();
    encrypted_file.sync_all().unwrap();
    encrypted_filename
}

fn decrypt_file(filename: &String, key: &String, iv: &String, cipher: Cipher) -> String {
    let mut file = File::open(filename).unwrap(); // Open file
    let mut data = Vec::new(); // Vector to store data
    file.read_to_end(&mut data).unwrap(); // Read file to vector
    let decrypted = decrypt(cipher, key.as_bytes(), Some(iv.as_bytes()), &data).unwrap();
    let decrypted_filename = format!("dec_{}", filename); // Decrypted file name
    let mut decrypted_file = File::create(decrypted_filename.as_str()).unwrap();
    decrypted_file.write_all(&decrypted).unwrap();
    decrypted_file.sync_all().unwrap();
    decrypted_filename
}

fn main() {
    let filename = generate_benchmark_file(1);
    let key = String::from("0123456789ABCDEF"); // key must be known only to sender and receiver
    let iv = String::from("FEDCBA9876543210"); // iv don't need to be secret

    println!("Szyfrowanie AES 128 ECB");
    let start = std::time::Instant::now();
    let encrypted_filename = encrypt_file(&filename, &key, &iv, Cipher::aes_128_ecb());
    let end_encrypt = start.elapsed();
    let start_decrypt = std::time::Instant::now();
    decrypt_file(&encrypted_filename, &key, &iv, Cipher::aes_128_ecb());
    let end_decrypt = start_decrypt.elapsed();
    println!(
        "Czas szyfrowania: {:?}, czas deszyfrowania: {:?}",
        end_encrypt, end_decrypt
    );

    println!("Szyfrowanie AES 128 CBC");
    let start = std::time::Instant::now();
    let encrypted_filename = encrypt_file(&filename, &key, &iv, Cipher::aes_128_cbc());
    let end_encrypt = start.elapsed();
    let start_decrypt = std::time::Instant::now();
    decrypt_file(&encrypted_filename, &key, &iv, Cipher::aes_128_cbc());
    let end_decrypt = start_decrypt.elapsed();
    println!(
        "Czas szyfrowania: {:?}, czas deszyfrowania: {:?}",
        end_encrypt, end_decrypt
    );

    println!("Szyfrowanie AES 128 OFB");
    let start = std::time::Instant::now();
    let encrypted_filename = encrypt_file(&filename, &key, &iv, Cipher::aes_128_ofb());
    let end_encrypt = start.elapsed();
    let start_decrypt = std::time::Instant::now();
    decrypt_file(&encrypted_filename, &key, &iv, Cipher::aes_128_ofb());
    let end_decrypt = start_decrypt.elapsed();
    println!(
        "Czas szyfrowania: {:?}, czas deszyfrowania: {:?}",
        end_encrypt, end_decrypt
    );

    println!("Szyfrowanie AES 128 CFB");
    let start = std::time::Instant::now();
    let encrypted_filename = encrypt_file(&filename, &key, &iv, Cipher::aes_128_cfb1());
    let end_encrypt = start.elapsed();
    let start_decrypt = std::time::Instant::now();
    decrypt_file(&encrypted_filename, &key, &iv, Cipher::aes_128_cfb1());
    let end_decrypt = start_decrypt.elapsed();
    println!(
        "Czas szyfrowania: {:?}, czas deszyfrowania: {:?}",
        end_encrypt, end_decrypt
    );

    println!("Szyfrowanie AES 128 CTR");
    let start = std::time::Instant::now();
    let encrypted_filename = encrypt_file(&filename, &key, &iv, Cipher::aes_128_ctr());
    let end_encrypt = start.elapsed();
    let start_decrypt = std::time::Instant::now();
    decrypt_file(&encrypted_filename, &key, &iv, Cipher::aes_128_ctr());
    let end_decrypt = start_decrypt.elapsed();
    println!(
        "Czas szyfrowania: {:?}, czas deszyfrowania: {:?}",
        end_encrypt, end_decrypt
    );
}
