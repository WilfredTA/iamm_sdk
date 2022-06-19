use iamm_sdk_schemas::*;
use trampoline_sdk::ckb_types::packed::{OutPoint, CellInput};
use trampoline_sdk::contract::schema::MolConversion;
use trampoline_sdk::contract::{OutputRule, ContractField, RuleContext, RuleScope, ContractCellField, TransactionField};
use trampoline_sdk::ckb_types::{H256, bytes::Bytes};
use trampoline_sdk::ckb_types::prelude::*;
use trampoline_sdk::ckb_types::core::TransactionView;
use ckb_hash::blake2b_256;
use trampoline_sdk::types::script::Script;
use trampoline_sdk::types::query::{CellQuery, QueryStatement, CellQueryAttribute};
use ckb_jsonrpc_types::Byte32 as JsonByte32;


fn genesis_id_from(input: CellInput) -> GenesisId {
    let input = input.previous_output();
    let seed_tx_hash = input.tx_hash();
    let seed_idx = input.index();
    let mut seed = Vec::with_capacity(36);
    seed.extend_from_slice(seed_tx_hash.as_slice());
    seed.extend_from_slice(seed_idx.as_slice());
    let hash = blake2b_256(&seed);

    GenesisId::from_mol(hash.pack())
}

pub fn op_mint(minter_lock_script: Script) -> (Vec<Box<dyn Fn(TransactionView) -> CellQuery + 'static>>, Vec<OutputRule<NftArgs, IammNFT>>) {
    (vec![
        Box::new(move |_tx| -> CellQuery {
            CellQuery {
                _query: QueryStatement::Single(CellQueryAttribute::LockHash(
                    JsonByte32::from(minter_lock_script
                        .clone()
                        .calc_script_hash().pack()),
                )),
                _limit: 1,
            }
        }),
    ],
        vec![
        OutputRule::new(RuleScope::ContractField(
            ContractField::Data
        ),
        
        |ctx| -> NftContractField {
            let nft: NftContractField = ctx.load(ContractField::Data);
            
            if let ContractCellField::Data(mut nft_data) = nft.clone() {
                let inputs: ContractCellField<NftArgs, IammNFT> = ctx.load(RuleScope::TransactionField(TransactionField::Inputs));
                if let ContractCellField::Inputs(inps) = inputs {
                    nft_data.genesis_id = genesis_id_from(inps.first().unwrap().clone());
                    NftContractField::Data(nft_data)
                } else {
                    nft
                }
            } else {
                nft
            }
        })
    ])
}

pub fn mint_nft(minter_lock_script: Script, mut contract: IammNFTContract) -> IammNFTContract {
    let (mint_op_input_rules, mint_op_output_rules) = op_mint(minter_lock_script.clone());
    for op in mint_op_input_rules {
        contract.add_input_rule(op);
    }

    contract.output_rules = mint_op_output_rules;
    contract
}