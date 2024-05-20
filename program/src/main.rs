#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let n = sp1_zkvm::io::read::<i8>();

    let old_enough = old_enough(n);

    sp1_zkvm::io::commit(&old_enough);
}

fn old_enough(n: i8) -> bool {
    n >= 18
}
