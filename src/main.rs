use halo2_gadgets::poseidon::Pow5Config as PoseidonConfig;
use pasta_curves::pallas;

#[derive(Clone, Debug)]
pub struct Foo {
    poseidon_config: PoseidonConfig<pallas::Base, 3, 2>,
}

fn main() {
    println!("Hello, world!");
}
