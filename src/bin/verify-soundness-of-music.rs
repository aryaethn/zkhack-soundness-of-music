#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use ark_ff::One;
use prompt::{puzzle, welcome};
use soundness_of_music::data::puzzle_data;
use soundness_of_music::prover;
use soundness_of_music::verifier;
use soundness_of_music::PUZZLE_DESCRIPTION;

type Fr = <ark_bls12_381::Bls12_381 as ark_ec::PairingEngine>::Fr;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (circuit, setup) = puzzle_data();

    let public_inputs = [Fr::one(), Fr::one()];

    /* Your solution here! */
    
    // Use the fake_prove function to generate a fake proof
    let proof = prover::fake_prove(&public_inputs, 1, &setup); // 1 private input (z variable)
    
    // Verify the fake proof
    assert!(verifier::verify(&public_inputs, &setup, &proof));
    println!("Puzzle Solved!✅");
}
