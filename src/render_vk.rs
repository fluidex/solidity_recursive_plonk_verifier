use handlebars::to_json;
use std::collections::HashMap;

use franklin_crypto::bellman::pairing::{
    bn256::{Bn256 as NodeEngine, Fr},
    CurveAffine, Engine,
};
use franklin_crypto::bellman::plonk::{better_better_cs::setup::VerificationKey, domains::Domain};
use recursive_aggregation_circuit::circuit::RecursiveAggregationCircuitBn256;

pub(crate) fn rendered_key(
    recursive_vk: VerificationKey<NodeEngine, RecursiveAggregationCircuitBn256<'static>>,
) -> HashMap<String, serde_json::Value> {
    let mut map = HashMap::new();

    let domain_size = recursive_vk.n.next_power_of_two().to_string();
    map.insert("domain_size".to_owned(), to_json(domain_size));

    let num_inputs = recursive_vk.num_inputs.to_string();
    map.insert("num_inputs".to_owned(), to_json(num_inputs));

    let domain = Domain::<Fr>::new_for_size(recursive_vk.n.next_power_of_two() as u64).unwrap();
    let omega = domain.generator;
    map.insert("omega".to_owned(), to_json(render_scalar_to_hex(&omega)));

    for (i, c) in recursive_vk.gate_setup_commitments.iter().enumerate() {
        let rendered = render_g1_affine_to_hex::<NodeEngine>(c);

        for (j, rendered) in rendered.iter().enumerate() {
            map.insert(
                format!("gate_setup_commitment_{}_{}", i, j),
                to_json(rendered),
            );
        }
    }

    for (i, c) in recursive_vk.gate_selectors_commitments.iter().enumerate() {
        let rendered = render_g1_affine_to_hex::<NodeEngine>(c);

        for (j, rendered) in rendered.iter().enumerate() {
            map.insert(
                format!("gate_selector_commitment_{}_{}", i, j),
                to_json(rendered),
            );
        }
    }

    for (i, c) in recursive_vk.permutation_commitments.iter().enumerate() {
        let rendered = render_g1_affine_to_hex::<NodeEngine>(c);

        for (j, rendered) in rendered.iter().enumerate() {
            map.insert(
                format!("permutation_commitment_{}_{}", i, j),
                to_json(rendered),
            );
        }
    }

    for (i, c) in recursive_vk.non_residues.into_iter().enumerate() {
        let rendered = render_scalar_to_hex(&c);
        map.insert(format!("permutation_non_residue_{}", i), to_json(&rendered));
    }

    let rendered = render_g2_affine_to_hex(&recursive_vk.g2_elements[1]);

    map.insert("g2_x_x_c0".to_owned(), to_json(&rendered[0]));
    map.insert("g2_x_x_c1".to_owned(), to_json(&rendered[1]));
    map.insert("g2_x_y_c0".to_owned(), to_json(&rendered[2]));
    map.insert("g2_x_y_c1".to_owned(), to_json(&rendered[3]));

    // to_json(map)
    map
}

use crate::primitives::render_scalar_to_hex;

fn render_g1_affine_to_hex<E: Engine>(point: &E::G1Affine) -> [String; 2] {
    if point.is_zero() {
        return ["0x0".to_owned(), "0x0".to_owned()];
    }

    let (x, y) = point.into_xy_unchecked();
    [render_scalar_to_hex(&x), render_scalar_to_hex(&y)]
}

fn render_g2_affine_to_hex(point: &<NodeEngine as Engine>::G2Affine) -> [String; 4] {
    if point.is_zero() {
        return [
            "0x0".to_owned(),
            "0x0".to_owned(),
            "0x0".to_owned(),
            "0x0".to_owned(),
        ];
    }

    let (x, y) = point.into_xy_unchecked();

    [
        render_scalar_to_hex(&x.c0),
        render_scalar_to_hex(&x.c1),
        render_scalar_to_hex(&y.c0),
        render_scalar_to_hex(&y.c1),
    ]
}
