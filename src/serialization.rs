//! Common serialization utilities.
//!
//! From zksync, this module provides building blocks for serializing and deserializing
//! proof types to ethereum friendly form.

//new bellman inside franklin_crypto
use bellman::plonk::{
    better_better_cs::{cs::Circuit as NewCircuit, proof::Proof as NewProof},
    better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof as OldProof},
};
use ethabi::ethereum_types::U256;
use franklin_crypto::{bellman, bellman::pairing::bn256};
use serde::{ser::SerializeSeq, Serialize, Serializer};

use crate::ethereum_serializer;
use crate::types::AggregatedProof;

type Engine = bn256::Bn256;
//type Fr = bn256::Fr;

/*
    serialize the output to fit a func in contract with following the parameters:
    function verifyAggregatedBlockProof(
        uint256[] memory _recursiveInput,
        uint256[] memory _proof,
        uint8[] memory _vkIndexes,
        uint256[] memory _individualVksInputs,
        uint256[16] memory _subproofsLimbs
    )
*/

impl Serialize for AggregatedProof {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(5))?;
        let (input, serialized_proof) = serialize_new_proof(&self.proof);
        seq.serialize_element(&input)?;
        seq.serialize_element(&serialized_proof)?;
        let vk_inputs: Vec<U256> = self
            .individual_vk_inputs
            .iter()
            .map(ethereum_serializer::serialize_fe)
            .collect();
        seq.serialize_element(&self.individual_vk_idxs)?;
        seq.serialize_element(&vk_inputs)?;
        let subproofs_limbs: Vec<U256> = self
            .aggr_limbs
            .iter()
            .map(ethereum_serializer::serialize_fe)
            .collect();
        assert_eq!(subproofs_limbs.len(), 16);
        seq.serialize_element(&subproofs_limbs)?;

        seq.end()
    }
}

pub fn serialize_new_proof<C: NewCircuit<Engine>>(
    proof: &NewProof<Engine, C>,
) -> (Vec<U256>, Vec<U256>) {
    let mut inputs = vec![];
    for input in proof.inputs.iter() {
        inputs.push(ethereum_serializer::serialize_fe(input));
    }
    let mut serialized_proof = vec![];

    for c in proof.state_polys_commitments.iter() {
        let (x, y) = ethereum_serializer::serialize_g1(c);
        serialized_proof.push(x);
        serialized_proof.push(y);
    }

    let (x, y) =
        ethereum_serializer::serialize_g1(&proof.copy_permutation_grand_product_commitment);
    serialized_proof.push(x);
    serialized_proof.push(y);

    for c in proof.quotient_poly_parts_commitments.iter() {
        let (x, y) = ethereum_serializer::serialize_g1(c);
        serialized_proof.push(x);
        serialized_proof.push(y);
    }

    for c in proof.state_polys_openings_at_z.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    for (_, _, c) in proof.state_polys_openings_at_dilations.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    assert_eq!(proof.gate_setup_openings_at_z.len(), 0);

    for (_, c) in proof.gate_selectors_openings_at_z.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    for c in proof.copy_permutation_polys_openings_at_z.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.copy_permutation_grand_product_opening_at_z_omega,
    ));
    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.quotient_poly_opening_at_z,
    ));
    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.linearization_poly_opening_at_z,
    ));

    let (x, y) = ethereum_serializer::serialize_g1(&proof.opening_proof_at_z);
    serialized_proof.push(x);
    serialized_proof.push(y);

    let (x, y) = ethereum_serializer::serialize_g1(&proof.opening_proof_at_z_omega);
    serialized_proof.push(x);
    serialized_proof.push(y);

    (inputs, serialized_proof)
}

//grep from solidity_plonk_verifier
pub fn serialize_proof(
    proof: &OldProof<Engine, PlonkCsWidth4WithNextStepParams>,
) -> (Vec<U256>, Vec<U256>) {
    let mut inputs = vec![];
    for input in proof.input_values.iter() {
        inputs.push(ethereum_serializer::serialize_fe(input));
    }
    let mut serialized_proof = vec![];

    for c in proof.wire_commitments.iter() {
        let (x, y) = ethereum_serializer::serialize_g1(c);
        serialized_proof.push(x);
        serialized_proof.push(y);
    }

    let (x, y) = ethereum_serializer::serialize_g1(&proof.grand_product_commitment);
    serialized_proof.push(x);
    serialized_proof.push(y);

    for c in proof.quotient_poly_commitments.iter() {
        let (x, y) = ethereum_serializer::serialize_g1(c);
        serialized_proof.push(x);
        serialized_proof.push(y);
    }

    for c in proof.wire_values_at_z.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    for c in proof.wire_values_at_z_omega.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.grand_product_at_z_omega,
    ));
    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.quotient_polynomial_at_z,
    ));
    serialized_proof.push(ethereum_serializer::serialize_fe(
        &proof.linearization_polynomial_at_z,
    ));

    for c in proof.permutation_polynomials_at_z.iter() {
        serialized_proof.push(ethereum_serializer::serialize_fe(c));
    }

    let (x, y) = ethereum_serializer::serialize_g1(&proof.opening_at_z_proof);
    serialized_proof.push(x);
    serialized_proof.push(y);

    let (x, y) = ethereum_serializer::serialize_g1(&proof.opening_at_z_omega_proof);
    serialized_proof.push(x);
    serialized_proof.push(y);

    (inputs, serialized_proof)
}
