#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_variables)]

imports!();

#[elrond_wasm_derive::callable(TestProxyImpl)]
pub trait TestProxy {
    #[callback(transferFromCallback)]
    fn transferFrom(
        &self,
        amountt: BigUint // if this is `amount`it breaks, anything else seems to work
    );
}

#[elrond_wasm_derive::contract(ContractImpl)]
pub trait Contract { 
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn trigger_remote_call(&self, calee_address: Address, value: BigUint) {
        let target_contract = contract_proxy!(self, &calee_address, TestProxy);
        target_contract.transferFrom(
            value
        );
    }

    #[endpoint]
    fn transferFrom(&self, amount: &BigUint) {
        self.remote_call_received(amount);
    }

    #[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
    fn remote_call_received(&self, value: &BigUint);
}