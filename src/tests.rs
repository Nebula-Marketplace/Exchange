use std::str::FromStr;

use cosmwasm_std::{Addr, Uint128, Decimal, Empty, coins};
use cw_multi_test::{App, ContractWrapper, Executor};
use nft_multi_test::{self, cw721_contract};

use crate::{contract::*, msg::{InstantiateMsg, Creator, ExecuteMsg}, ContractError};

type Extension = Option<Empty>;

#[test]
fn init() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");
}

#[test]
fn list() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    let exchange = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::Approve { 
            token_id: 0.to_string(),
            spender: String::from(&exchange),
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        exchange,
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).expect("could not list");
}

#[test]
fn buy() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    let exchange = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::Approve { 
            token_id: 0.to_string(),
            spender: String::from(&exchange),
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).expect("could not list");

    app.execute_contract(
        Addr::unchecked("buyer"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::Buy {
            id: 0.to_string(),
        },
        &coins(1010000, "inj"),
    ).expect("could not buy");
}

#[test]
fn dup_listing() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    let exchange = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::Approve { 
            token_id: 0.to_string(),
            spender: String::from(&exchange),
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    // should pass
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).expect("could not list");

    let err: ContractError = app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).unwrap_err().downcast().unwrap();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn delist_deauth() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    let exchange = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::Approve { 
            token_id: 0.to_string(),
            spender: String::from(&exchange),
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).expect("could not list");

    let err: ContractError = app.execute_contract(
        Addr::unchecked("bad_actor"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::DeList {
            id: 0.to_string(),
        },
        &vec![]
    ).unwrap_err().downcast().unwrap();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn delist() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_code = cw721_contract();
    let nft_code_id = app.store_code(nft_code);
    let nft = nft_multi_test::instantiate(&mut app, nft_code_id, &nft_multi_test::InstantiateMsg {
        name: "Test Collection".to_string(),
        symbol: "TEST".to_string(),
        minter: "owner".to_string(),
    }).expect("Could not instantiate nft contract");

    let exchange = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: (&nft).to_string(),
            description: "Test collection on Nebula".to_string(),
            symbol: "TEST".to_string(),
            logo_uri: "https://example.com/logo.png".to_string(),
            banner_uri: "https://example.com/banner.png".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::Approve { 
            token_id: 0.to_string(),
            spender: String::from(&exchange),
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::List {
            id: 0.to_string(),
            price: Uint128::new(1000000),
            expires: 0
        },
        &vec![],
    ).expect("could not list");

    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&exchange),
        &ExecuteMsg::DeList {
            id: 0.to_string(),
        },
        &vec![]
    ).expect("could not delist");
}

#[test]
fn cw_math_platform_fee() {
    let payment = Uint128::new(1000000000000000000);
    let fee = Decimal::percent(3);
    let fee_amount = payment * fee;

    assert_eq!(fee_amount, Uint128::new(30000000000000000));
}

#[test]
fn cw_math_royalties() {
    let payment = Uint128::new(1069000000000000000);
    let price = Uint128::new(1000000000000000000);
    let fee = Decimal::percent(3);
    let royalties = Decimal::from_ratio(690 as u32, 10_000u32);
    let royalty = payment - price;
    let fee = price * fee;
    assert_eq!(royalty, price * royalties);
    assert_eq!(royalty, Uint128::new(69000000000000000));
    assert_eq!(fee, Uint128::new(30000000000000000));
}