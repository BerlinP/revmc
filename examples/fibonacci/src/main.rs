use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{address, hex, AccountInfo, Bytecode, TransactTo, U256}, Database,
};
use revmc_examples_fibonacci::build_evm;
use revmc_examples_fibonacci::fibo::encode_fib;

include!("./common.rs");

fn main() {
    let now = Instant::now();
    let db = CacheDB::new(EmptyDB::new());
    let mut evm = build_evm(db);
    let fibo_address = address!("0000000000000000000000000000000000001234");
    evm.db_mut().insert_account_info(
        fibo_address,
        AccountInfo {
            code_hash: FIBO_HASH.into(),
            code: Some(Bytecode::new_raw(FIBO_CODE.into())),
            ..Default::default()
        },
    );

    let call_data = encode_fib(U256::from(100));
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibo_address);
    evm.context.evm.env.tx.data = call_data.into();

    use std::time::Instant;
    let result = evm.transact_commit().unwrap();
    println!("fib(100) = {}", U256::from_be_slice(result.output().unwrap()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
