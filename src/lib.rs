//! Library to generate a EVM verifier contract

use handlebars::to_json;
use handlebars::Handlebars;
use std::collections::HashMap;

pub mod config;
mod primitives;
mod render_vk;
pub mod serialization;
pub mod types;
pub use crate::config::Config;
pub use primitives::ethereum_serializer;
pub use recursive_aggregation_circuit::circuit;

use crate::render_vk::rendered_key;
use primitives::render_scalar_to_hex;

pub fn create_verifier_contract_from_template(
    config: Config,
    template: &str,
    render_to_path: &str,
) {
    //    let template =
    //        std::fs::read_to_string(template_filepath).expect("failed to read Verifier template file");
    let mut template_params = HashMap::new();

    template_params.insert(
        "vk_tree_root".to_string(),
        to_json(render_scalar_to_hex(&config.vk_tree_root)),
    );

    template_params.insert("vk_max_index".to_string(), to_json(&config.vk_max_index));

    // template_params.insert("vk_max_index".to_string(), to_json(config.vk_max_index));

    // TODO: improve?
    let key_details = rendered_key(config.recursive_vk);
    for (k, v) in key_details {
        template_params.insert(k, to_json(v));
    }

    let res = Handlebars::new()
        .render_template(template, &template_params)
        .expect("failed to render Verifiers.sol template");
    std::fs::write(render_to_path, res).expect("failed to wrtie Verifier.sol");
    log::info!("Verifier contract successfully generated");
}

pub fn create_verifier_contract(config: Config, template_filepath: &str, render_to_path: &str) {
    let template =
        std::fs::read_to_string(template_filepath).expect("failed to read Verifier template file");
    create_verifier_contract_from_template(config, &template, render_to_path)
}

pub fn create_verifier_contract_from_default_template(config: Config, render_to_path: &str) {
    let template = include_str!("./VerifierTemplate.sol");
    create_verifier_contract_from_template(config, template, render_to_path)
}
