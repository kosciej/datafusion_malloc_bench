use std::sync::Arc;

use datafusion::prelude::*;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::test]
async fn test1() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();

    let data = datafusion_malloc_bench::create_data_table();

    ctx.register_table("test", Arc::new(data)).unwrap();

    let _profiler = dhat::Profiler::new_heap();
    // create a plan to run a SQL query
    let df = ctx.sql("SELECT sum(a), sum(b), sum(c) FROM test;").await?;
    // execute and print results
    df.show().await?;
    drop(_profiler);

    Ok(())
}
