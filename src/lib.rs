use std::sync::Arc;

use criterion::{black_box, Criterion};
use datafusion::{
    arrow::{
        array::Int32Array,
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    datasource::MemTable,
    from_slice::FromSlice,
    prelude::SessionContext,
};

pub fn create_data_table() -> MemTable {
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Int32, false),
        Field::new("c", DataType::Int32, false),
        Field::new("d", DataType::Int32, true),
    ]));

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int32Array::from_slice([1, 2, 3])),
            Arc::new(Int32Array::from_slice([4, 5, 6])),
            Arc::new(Int32Array::from_slice([7, 8, 9])),
            Arc::new(Int32Array::from(vec![None, None, Some(9)])),
        ],
    )
    .unwrap();

    MemTable::try_new(schema, vec![vec![batch]]).unwrap()
}

pub fn benchmark(c: &mut Criterion, name: &str, query: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = SessionContext::new();

    let data = create_data_table();

    ctx.register_table("test", Arc::new(data)).unwrap();

    c.bench_function(name, |b| {
        b.to_async(&rt).iter(|| async {
                let r = ctx.sql(query).await.unwrap();
                black_box(r);
            });
    });
}

pub fn select_all_benchmark(c: &mut Criterion, name: &str) {
    benchmark(c, name, "SELECT sum(a), sum(b), sum(c) FROM test;")
}
