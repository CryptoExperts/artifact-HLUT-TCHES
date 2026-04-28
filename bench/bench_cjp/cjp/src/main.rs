use std::{collections::HashMap, time::Instant};

use tfhe::shortint::parameters::v0_10::classic::gaussian::p_fail_2_minus_64::ks_pbs::*;
use tfhe::shortint::prelude::*;

const PARAMETERS: [ClassicPBSParameters; 7] = [
    V0_10_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M64,
    V0_10_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M64,
];

// Generate the client key and the server key:
fn benchmark_pbs_cjp(n: usize, parameters: ClassicPBSParameters) -> u128 {
    // Generate the client key and the server key:
    let (cks, sks) = gen_keys(parameters);

    let msg: u64 = 1;
    let ct = cks.encrypt(msg);
    let modulus = cks.parameters().message_modulus().0 as u64;

    // Generate the lookup table for the function f: x -> x*x*x mod 4
    let lut = sks.generate_lookup_table(|x| x * x * x % modulus);
    let start = Instant::now();
    let _ = sks.apply_lookup_table(&ct, &lut);
    let stop = start.elapsed();
    stop.as_millis()
}

fn main() {
    let mut results = HashMap::new();

    for (n, params) in (2..).zip(PARAMETERS.into_iter()) {
        let timing = benchmark_pbs_cjp(n, params);
        results.insert(n.to_string(), timing);
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../data/regenerated/cjp21.json");
    std::fs::write(path, serde_json::to_string(&results).unwrap()).unwrap();
}
