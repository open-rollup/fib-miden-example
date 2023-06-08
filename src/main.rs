use miden::{
    prove, Assembler, MemAdviceProvider, ProofOptions, 
	utils::Serializable,
};
use sp_core::H256;
// use sp_core::hexdisplay::ascii_format;
use std::fs;

use pallet_open_rollup::verifier::{Verifier, MidenVerifier, raw_inputs_to_stack_inputs};

fn main() {
    let program_source= fs::read_to_string("miden.masm").unwrap();

    let assembler = Assembler::default();
    let program = assembler.compile(program_source).unwrap();
    let program_hash = H256::from_slice(&program.hash().as_bytes());

    // state_root
    let old_state_root = H256::repeat_byte(0);
    let inputs = raw_inputs_to_stack_inputs(old_state_root.as_bytes()).unwrap();

    let (outputs, proof) =
        prove(&program, inputs, MemAdviceProvider::default(), ProofOptions::default()).unwrap();

    println!(
        "verify result: {:?}",
        MidenVerifier::verify(
            program_hash.as_bytes(),
            old_state_root.as_bytes(),
            &proof.to_bytes(),
            &outputs.to_bytes()
        )
    );

    fs::write("miden_program_hash.txt", format!("{:?}", program_hash)).unwrap();
    fs::write("miden.proof", proof.to_bytes()).unwrap();
    fs::write("miden.outputs", outputs.to_bytes()).unwrap();

    // fs::write("miden.proof.txt", ascii_format(&proof.to_bytes())).unwrap();
    // fs::write("miden.outputs.txt", ascii_format(&outputs.to_bytes())).unwrap();

    println!("program_hash: {:?}", program_hash);
    println!("proof sieze: {}", proof.to_bytes().len());
    println!("outputs size: {}", outputs.to_bytes().len());

}
