//Inspired from zksync
//convert primitives to ethereum friendly type

use franklin_crypto::bellman::pairing::{
    bn256::Bn256,
    ff::{PrimeField, PrimeFieldRepr, ScalarEngine},
    CurveAffine, Engine,
};

pub(crate) fn render_scalar_to_hex<F: PrimeField>(el: &F) -> String {
    let mut buff = vec![];
    let repr = el.into_repr();
    repr.write_be(&mut buff).unwrap();

    format!("0x{}", hex::encode(buff))
}

pub mod ethereum_serializer {
    use super::*;
    use ethabi::ethereum_types::U256;

    pub fn serialize_g1(point: &<Bn256 as Engine>::G1Affine) -> (U256, U256) {
        if point.is_zero() {
            return (U256::zero(), U256::zero());
        }
        let uncompressed = point.into_uncompressed();

        let uncompressed_slice = uncompressed.as_ref();

        // bellman serializes points as big endian and in the form x, y
        // ethereum expects the same order in memory
        let x = U256::from_big_endian(&uncompressed_slice[0..32]);
        let y = U256::from_big_endian(&uncompressed_slice[32..64]);

        (x, y)
    }

    pub fn serialize_g2(point: &<Bn256 as Engine>::G2Affine) -> ((U256, U256), (U256, U256)) {
        let uncompressed = point.into_uncompressed();

        let uncompressed_slice = uncompressed.as_ref();

        // bellman serializes points as big endian and in the form x1*u, x0, y1*u, y0
        // ethereum expects the same order in memory
        let x_1 = U256::from_big_endian(&uncompressed_slice[0..32]);
        let x_0 = U256::from_big_endian(&uncompressed_slice[32..64]);
        let y_1 = U256::from_big_endian(&uncompressed_slice[64..96]);
        let y_0 = U256::from_big_endian(&uncompressed_slice[96..128]);

        ((x_1, x_0), (y_1, y_0))
    }

    pub fn serialize_fe(field_element: &<Bn256 as ScalarEngine>::Fr) -> U256 {
        let mut be_bytes = [0u8; 32];
        field_element
            .into_repr()
            .write_be(&mut be_bytes[..])
            .expect("get new root BE bytes");
        U256::from_big_endian(&be_bytes[..])
    }
}
