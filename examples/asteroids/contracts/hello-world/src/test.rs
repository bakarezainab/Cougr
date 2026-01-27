#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_smoke_and_init() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let count = client.cougr_smoke();
    assert_eq!(count, 1);

    client.init_game();
    assert_eq!(client.get_score(), 0);
    assert_eq!(client.check_game_over(), false);
}

#[test]
fn test_tick_progression() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.init_game();
    client.rotate_ship(&1);
    client.thrust_ship();
    client.shoot();
    client.update_tick();
}
