// crates/fingerprint/examples/lagrange.rs

use fingerprint::codeword_lagrange;

fn main() {
    let p = 11;
    // Anchor points for interpolation: x = 0, 1, 2
    let xs = vec![0, 1, 2];
    let messages = [("a", vec![2, 1, 1]), ("b", vec![2, 1, 0])];
    println!("Field F_p with p = {p} and {:?}", messages);
    for (name, message) in &messages {
        // Generate the full F_p codeword via Lagrange interpolation
        let cw = codeword_lagrange(message, &xs, p);
        println!("Lagrange codeword for {}: {:?}", name, cw);
    }
}
