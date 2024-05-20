use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::path::Path;
use std::process::Command;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Path to the JPEG image
    let image_path = "age.jpg";

    // Extract the age from the image
    let age = extract_age_from_image(image_path).expect("Failed to extract age from image");

    // Create an input stream
    let mut stdin = SP1Stdin::new();
    stdin.write(&age);

    // Generate and verify the proof
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).unwrap();

    let old_enough = proof.public_values.read::<bool>();
    println!("Person is old enough? {}, Number: {}", old_enough, age);

    client.verify(&proof, &vk).expect("verification failed");

    // Save the proof
    proof
        .save("proof-with-is-min-age.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}

fn extract_age_from_image<P: AsRef<Path>>(image_path: P) -> Result<i8, Box<dyn std::error::Error>> {
    // Call the Tesseract CLI to perform OCR on the image
    let output = Command::new("tesseract")
        .arg(image_path.as_ref())
        .arg("stdout")
        .arg("--psm")
        .arg("6") // Assume a single uniform block of text
        .output()?;

    if !output.status.success() {
        return Err(format!("Tesseract command failed with status: {}", output.status).into());
    }

    // Get the OCR result as a string
    let text = String::from_utf8(output.stdout)?;

    // Print the extracted text for debugging
    println!("Extracted text: {}", text);

    // Extract age from OCR result
    let trimmed_text = text.trim();
    let filtered_text: String = trimmed_text
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let age: i8 = filtered_text.parse()?;
    Ok(age)
}
