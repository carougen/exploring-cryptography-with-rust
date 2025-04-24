// crates/fingerprint/examples/polynomial.rs

use fingerprint::codeword_polynomial;

fn main() {
    let p = 11;
    let messages = [("a", vec![2, 1, 1]), ("b", vec![2, 1, 0])];
    println!("Field F_p with p = {p} and {:?}", messages);
    for (name, message) in &messages {
        // Generate the full F_p codeword via polynomial evaluation
        let cw = codeword_polynomial(message, p);
        println!("Polynomial codeword for {}: {:?}", name, cw);
    }
}
