#![forbid(unsafe_code)]

fn main() {
    let commands = [
        "validate-scenario",
        "capture-run",
        "fingerprint-env",
        "emit-bundle",
        "emit-diff-seed",
        "validate-handoff",
    ];

    println!("dna-xl-obs scaffold");
    println!("available commands: {}", commands.join(", "));
}
