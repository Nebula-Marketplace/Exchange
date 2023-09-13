use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

use serde::{Deserialize, Serialize};

use crate::{contract::*, msg::{InstantiateMsg, Creator}};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Empty {}

#[test]
fn init() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: "contract".to_string(),
            description: "description".to_string(),
            symbol: "symbol".to_string(),
            logo_uri: "logo_uri".to_string(),
            banner_uri: "banner_uri".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Contract", 
        None
    ).expect("contract failed to instantiate");
}