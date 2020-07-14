
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_variables)]

imports!();

#[elrond_wasm_derive::callable(TransferFromProxy)]
pub trait TransferFrom {
    #[callback(transferFromCallback)]
    fn transferFrom(&self, sender: &Address, recipient: &Address, amount: BigUint);
}

#[elrond_wasm_derive::contract(ContractImpl)]
pub trait Contract {

    #[init]
    fn init(&self) {
    }

    #[callback]
    fn transferFromCallback(&self, _call_result: AsyncCallResult<()>) {
    }

}
