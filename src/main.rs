use miden::{
    prove, Assembler, MemAdviceProvider, ProofOptions, StackInputs, Kernel, ProgramInfo,
	utils::{ByteReader, Serializable, SliceReader},
};
use std::fs;

/// Convert bytes to Miden's `StackInputs`.
pub fn raw_inputs_to_stack_inputs(raw_data: &[u8]) -> StackInputs {
	let miden_inputs;
	{
		let mut inputs_reader = SliceReader::new(raw_data);
		let mut stack = Vec::new();
		for _ in 0..4 {
			stack.push(inputs_reader.read_u64().unwrap())
		}
		miden_inputs = StackInputs::try_from_values(stack).unwrap();
	}

	miden_inputs
}

fn main() {
    let program_source= fs::read_to_string("miden.masm").unwrap();

    let assembler = Assembler::default();
    let program = assembler.compile(program_source).unwrap();
    // let program_hash = H256::from_slice(&program.hash().as_bytes());
    let program_hash = program.hash().as_bytes();

    // state_root
    let old_state_root: [u8; 32] = [0; 32];
    let inputs = raw_inputs_to_stack_inputs(&old_state_root);

    let (outputs, proof) =
        prove(&program, inputs.clone(), MemAdviceProvider::default(), ProofOptions::default()).unwrap();
    
    let program_info = ProgramInfo::new(program.hash(), Kernel::default());

    // let proof = ExecutionProof::from_bytes(&proof.to_bytes()).unwrap();
    // println!("proof: {:?}", proof);

    println!("{:?}", miden::verify(program_info, inputs, outputs.clone(), proof.clone()));

    let outputs = outputs.to_bytes();
    let proof = proof.to_bytes();

    // check proof
    println!("MidenVerifier: \n{:?}\n{:?}\n{:?}\n{:?}\n", program_hash, old_state_root, outputs.len(), proof.len());
    println!("outputs: {:?}", outputs);
    println!("proof start: {:?}", &proof[..10]);
    println!("proof end: {:?}", &proof[proof.len()-10..]);

    fs::write("miden_program_hash.txt", format!("0x{:?}", hex::encode(program_hash))).unwrap();
    fs::write("miden.proof", proof.clone()).unwrap();
    fs::write("miden.outputs", outputs.clone()).unwrap();

    println!("program_hash: 0x{:?}", hex::encode(program_hash));
    println!("proof sieze: {}", proof.len());
    println!("outputs size: {}", outputs.len());

}
