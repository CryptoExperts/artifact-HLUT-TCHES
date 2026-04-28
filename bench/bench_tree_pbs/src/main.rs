use serde::Serialize;
use std::collections::HashMap;
use std::time::Instant;
use tfhe::gadget::prelude::*;
use tfhe::shortint::parameters::{
    ClassicPBSParameters, PARAM_MESSAGE_2_CARRY_0_KS_PBS, PARAM_MESSAGE_3_CARRY_0_KS_PBS,
    PARAM_MESSAGE_4_CARRY_0_KS_PBS, PARAM_MESSAGE_5_CARRY_0_KS_PBS, PARAM_MESSAGE_6_CARRY_0_KS_PBS,
    PARAM_MESSAGE_7_CARRY_0_KS_PBS,
};

const PARAMETERS: [ClassicPBSParameters; 6] = [
    PARAM_MESSAGE_2_CARRY_0_KS_PBS,
    PARAM_MESSAGE_3_CARRY_0_KS_PBS,
    PARAM_MESSAGE_4_CARRY_0_KS_PBS,
    PARAM_MESSAGE_5_CARRY_0_KS_PBS,
    PARAM_MESSAGE_6_CARRY_0_KS_PBS,
    PARAM_MESSAGE_7_CARRY_0_KS_PBS,
];

pub trait ToBooleanParameters {
    fn to_boolean_parameters(&self) -> BooleanParameters;
}

impl ToBooleanParameters for ClassicPBSParameters {
    fn to_boolean_parameters(&self) -> BooleanParameters {
        BooleanParameters {
            lwe_dimension: self.lwe_dimension,
            glwe_dimension: self.glwe_dimension,
            polynomial_size: self.polynomial_size,
            lwe_modular_std_dev: self.lwe_modular_std_dev,
            glwe_modular_std_dev: self.glwe_modular_std_dev,
            pbs_base_log: self.pbs_base_log,
            pbs_level: self.pbs_level,
            ks_base_log: self.ks_base_log,
            ks_level: self.ks_level,
            encryption_key_choice: self.encryption_key_choice,
        }
    }
}

fn benchmark_tree_pbs_two_blocks(
    log_s: usize,
    s: u64,
    p: u64,
    n: usize,
    parameters: &ClassicPBSParameters,
) -> u128 {
    // Generate the client key and the server key:
    let (cks, sks) = gen_keys(&parameters.to_boolean_parameters());
    let encoding = Encoding::new_canonical(s, (0..s).collect(), p);
    let encodings_out = vec![encoding.clone(); n];

    let clear_inputs = vec![0; n];

    let inputs = clear_inputs
        .iter()
        .map(|msg| cks.encrypt_arithmetic(*msg, &encoding))
        .collect();

    let start = Instant::now();
    let _ = sks.full_tree_bootstrapping(
        &inputs,
        &encodings_out,
        1 << (log_s * n),
        &|x| x,
        &cks,
        false,
    );
    let stop = start.elapsed();
    println!("2 blocks of {} bits took {:?}", log_s, stop);
    stop.as_millis()
}

fn benchmark_tree_pbs_three_blocks(
    log_s: usize,
    s: u64,
    p: u64,
    n: usize,
    parameters: &ClassicPBSParameters,
) -> u128 {
    // Generate the client key and the server key:
    let (cks, sks) = gen_keys(&parameters.to_boolean_parameters());
    let encoding = Encoding::new_canonical(s, (0..s).collect(), p);
    let encodings_out = vec![encoding.clone(); n];

    let clear_inputs = vec![0; n];

    let inputs = clear_inputs
        .iter()
        .map(|msg| cks.encrypt_arithmetic(*msg, &encoding))
        .collect();

    let start = Instant::now();
    let _ = sks.full_tree_bootstrapping_three_blocks(
        &inputs,
        &encodings_out,
        1 << (log_s * n),
        &|x| x,
        &cks,
        false,
    );
    let stop = start.elapsed();
    println!("3 blocks of {} bits took {:?}", log_s, stop);
    stop.as_millis()
}

fn main() {
    let mut results = HashMap::new();
    results.insert(String::from("2 blocks"), HashMap::<String, u128>::new());
    results.insert(String::from("3 blocks"), HashMap::<String, u128>::new());
    for (log_s, params) in (2..).zip(PARAMETERS.iter()) {
        let s = 1 << log_s;
        let timing = benchmark_tree_pbs_two_blocks(log_s, s, s + 1, 2, params);
        results
            .get_mut("2 blocks")
            .unwrap()
            .insert((log_s * 2).to_string(), timing);
    }
    for (log_s, params) in (2..6).zip(PARAMETERS.iter()) {
        let s = 1 << log_s;
        let timing = benchmark_tree_pbs_three_blocks(log_s, s, s + 1, 3, params);
        results
            .get_mut("3 blocks")
            .unwrap()
            .insert((log_s * 3).to_string(), timing);
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/regenerated/timings_tbm.json");
    std::fs::write(path, serde_json::to_string(&results).unwrap()).unwrap();
}
