use franklin_crypto::bellman::pairing::bn256::{Bn256, Fr};
use franklin_crypto::bellman::plonk::better_better_cs::setup::VerificationKey;
use recursive_aggregation_circuit::circuit::RecursiveAggregationCircuitBn256;

pub struct Config {
    pub recursive_vk: VerificationKey<Bn256, RecursiveAggregationCircuitBn256<'static>>, // TODO: fix type
    pub vk_tree_root: Fr,
    //    pub vk_max_index: u8,
    pub vk_input_num: usize,
}
