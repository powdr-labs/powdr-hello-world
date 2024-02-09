use powdr::backend::BackendType;
use powdr::Bn254Field;
use powdr::Pipeline;

use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn main() {
    // Straightforward case
    let _proof = Pipeline::<Bn254Field>::default()
        .from_file("hello_world.asm".into())
        .with_prover_inputs(vec![0.into()])
        .with_backend(BackendType::Halo2)
        .proof()
        .unwrap();

    // Step-by-step case

    // First we create the universal setup of size 8 needed by Halo2
    let backend = BackendType::Halo2.factory::<Bn254Field>().create(8);

    let mut params_file = BufWriter::new(File::create("params.bin").unwrap());
    backend.write_setup(&mut params_file).unwrap();
    params_file.flush().unwrap();

    // Configure a pipeline
    let mut pipeline = Pipeline::<Bn254Field>::default()
        .from_file("hello_world.asm".into())
        .with_prover_inputs(vec![0.into()])
        .with_backend(BackendType::Halo2)
        .with_setup_file(Some("params.bin".into()));

    // Create the verification key
    let vkey = pipeline.verification_key().unwrap();
    fs::write("vkey.bin", vkey).unwrap();

    // Add the verification key and create a proof
    let proof = pipeline
        .clone()
        .with_vkey_file(Some("vkey.bin".into()))
        .proof()
        .unwrap()
        .proof
        .unwrap();

    // Create a fresh pipeline for separate proof verification
    let mut pipeline = pipeline
        .with_backend(BackendType::Halo2)
        .with_setup_file(Some("params.bin".into()))
        .with_vkey_file(Some("vkey.bin".into()));

    pipeline.verify(proof, &[vec![]]).unwrap();
}
