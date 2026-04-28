#![allow(dead_code)]

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use tfhe::float_wopbs::gen_keys;

#[allow(unused_imports)]
use tfhe::shortint::{prelude::*, WopbsParameters};

macro_rules! named_param {
    ($param:ident) => {
        (stringify!($param), $param)
    };
}

criterion_main!(float_hlut);

criterion_group!{name=float_hlut; config = Criterion::default()
       .sample_size(10)
       .warm_up_time(Duration::from_secs(1));
   targets = benchmark_wopbs}


// #8 in Table 2 in https://link.springer.com/10.1007/s00145-023-09463-5
// Naming: 1 bit per ciphertext, 16 ciphertexts
pub const PARAM_WOP_PBS_1_16: WopbsParameters = WopbsParameters {
    lwe_dimension: LweDimension(549),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0003177104139262535),
    glwe_modular_std_dev: StandardDev(3.162026630747649e-16),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(17),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(1),
    cbs_base_log: DecompositionBaseLog(13),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(1),
    encryption_key_choice: EncryptionKeyChoice::Big,
    ciphertext_modulus: CiphertextModulus::new_native(),
};


pub const PARAM_WOP_PBS_2_8: WopbsParameters = WopbsParameters {
    lwe_dimension: LweDimension(534),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0004192214045106218),
    glwe_modular_std_dev: StandardDev(3.162026630747649e-16),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(17),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(2),
    cbs_base_log: DecompositionBaseLog(9),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
    encryption_key_choice: EncryptionKeyChoice::Big,
    ciphertext_modulus: CiphertextModulus::new_native(),
};


pub const PARAM_WOP_PBS_4_4: WopbsParameters = WopbsParameters {
    lwe_dimension: LweDimension(538),
    glwe_dimension: GlweDimension(4),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00038844554870845634),
    glwe_modular_std_dev: StandardDev(2.168404344971009e-19),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(4),
    ks_level: DecompositionLevelCount(10),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(20),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(7),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
    encryption_key_choice: EncryptionKeyChoice::Big,
    ciphertext_modulus: CiphertextModulus::new_native(),
};



pub fn benchmark_wopbs(c: &mut Criterion) {
    for n_blocks in [1, 2, 4]{
        for precision_lut in (4..16){
            if (precision_lut % n_blocks == 0){

                let bits_per_block = precision_lut / n_blocks;

                let mut params = match n_blocks {
                    1 => PARAM_WOP_PBS_1_16,
                    2 =>PARAM_WOP_PBS_2_8,
                    4 => PARAM_WOP_PBS_4_4,
                    _ => panic!()
                };

                params.message_modulus = MessageModulus(1 << bits_per_block);

                let (cks, sks) = gen_keys(params);
                let msg_1 = 1;


                let clear_blocks: Vec<u64> = (0..n_blocks).map(|i| (msg_1 >> (i * bits_per_block)) % (1 << bits_per_block)).collect();


                // Encryption:
                let mut ciphertexts = clear_blocks.iter().map(|clear_block| cks.key.encrypt  (*clear_block)).collect(); // Here I've changed the key field to public



                let lut = sks.create_lut_for_benchmark(&mut ciphertexts);

                let bench_id = format!("WOP-PBS with n_blocks = {n_blocks} and precision_lut = {precision_lut}");
                c.bench_function(&bench_id, |b| {
                    b.iter(|| {
                        sks.wop_pbs_to_benchmark(&sks, &mut ciphertexts, &lut, precision_lut);
                    })
                });
            }
        }
    }
}
