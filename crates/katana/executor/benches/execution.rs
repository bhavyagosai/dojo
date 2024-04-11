use criterion::{black_box, criterion_group, criterion_main, Criterion};
use katana_executor::{
    implementation::blockifier::BlockifierFactory, ExecutorFactory, SimulationFlag,
};
use katana_primitives::{
    contract::ContractAddress, env::CfgEnv, transaction::ExecutableTxWithHash,
};
use katana_provider::traits::state::StateProvider;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = measure_tx_execution
}

criterion_main!(benches);

fn measure_tx_execution(c: &mut Criterion) {
    let sender = ContractAddress::default();

    let transactions = generate_transactions(sender, 100);
    let in_memory_db = set_up_in_memory_state();
    let persistent_db = set_up_db_state();

    let cfg = CfgEnv::default();
    let flags = SimulationFlag { skip_validate: true, ..Default::default() };

    c.bench_function("Blockifier.InMemoryDB", |b| {
        let factory = BlockifierFactory::new(cfg.clone(), flags.clone());
        let mut executor = factory.with_state(&in_memory_db);
        b.iter(|| {
            let _ = executor.execute_transactions(black_box(transactions.clone()));
        })
    });

    c.bench_function("Blockifier.OnDiskDB", |b| {
        let factory = BlockifierFactory::new(cfg.clone(), flags.clone());
        let mut executor = factory.with_state(&persistent_db);
        b.iter(|| {
            let _ = executor.execute_transactions(black_box(transactions.clone()));
        })
    });
}

pub(crate) fn generate_transactions(
    sender: ContractAddress,
    count: u64,
) -> Vec<ExecutableTxWithHash> {
    todo!()
}
