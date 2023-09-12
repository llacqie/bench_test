use artemis_core::{
    engine::Engine,
    types::{Collector, CollectorStream, Executor, Strategy},
};
use async_trait::async_trait;

struct NCollector;

#[async_trait]
impl Collector<u64> for NCollector {
    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, u64>> {
        let stream = futures::stream::iter(0..10000000u64);

        Ok(Box::pin(stream))
    }
}

struct NStrategy;

#[async_trait]
impl Strategy<u64, u64> for NStrategy {
    async fn sync_state(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn process_event(&mut self, event: u64) -> Vec<u64> {
        vec![event]
    }
}

struct NExecutor;

#[async_trait]
impl Executor<u64> for NExecutor {
    async fn execute(&self, _: u64) -> anyhow::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut engine: Engine<u64, u64> = Engine::default();

    let now = tokio::time::Instant::now();

    engine.add_collector(Box::new(NCollector));
    engine.add_strategy(Box::new(NStrategy));
    engine.add_executor(Box::new(NExecutor));

    if let Ok(mut set) = engine.run().await {
        set.join_next().await.unwrap().unwrap()
    }

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
