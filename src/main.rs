use rand_core::OsRng;
use dusk_plonk::prelude::*;

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a < 2^6
// 3) b < 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input
#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    // e: JubJubScalar,
    // f: JubJubAffine,
}

impl TestCircuit {
    fn fill() -> Self {
        Self {
            a: BlsScalar::from(5u64), 
            b: BlsScalar::from(25u64),
            c: BlsScalar::from(30u64),
            d: BlsScalar::from(125u64) 
        }
    }
    
}
impl Circuit for TestCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint =
            Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 6);
        composer.component_range(b, 6);

        // Make second constraint a * b = d
        let constraint =
            Constraint::new().mult(1).public(-self.d).a(a).b(b);

        composer.append_gate(constraint);

        // let e = composer.append_witness(self.e);
        // let scalar_mul_result = composer
        //     .component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED)?;

        // // Apply the constraint
        // composer.assert_equal_public_point(scalar_mul_result, self.f);

        Ok(())
    }
}


fn main() {
    let label = b"transcript-arguments";
    let pp = PublicParameters::setup(1 << 12, &mut OsRng)
        .expect("failed to setup");

    let (prover, verifier) = Compiler::compile::<TestCircuit>(&pp, label)
        .expect("failed to compile circuit");

    // Generate the proof and its public inputs
    let (proof, public_inputs) = prover
        .prove(&mut OsRng, &TestCircuit::fill())
        .expect("failed to prove");

    // Verify the generated proof
    verifier
        .verify(&proof, &public_inputs)
        .expect("failed to verify proof");
    println!("finish verify. all pass")
}
