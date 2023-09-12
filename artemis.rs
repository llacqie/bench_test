use artemis_core::{
    engine::Engine,
    types::{Collector, CollectorStream, Executor, Strategy},
};
use async_trait::async_trait;

const SIZE: usize = 10000000usize;

struct NCollector;

#[async_trait]
impl Collector<usize> for NCollector {
    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, usize>> {
        let stream = futures::stream::iter(0..SIZE);

        Ok(Box::pin(stream))
    }
}

struct NStrategy;

#[async_trait]
impl Strategy<usize, usize> for NStrategy {
    async fn sync_state(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn process_event(&mut self, event: usize) -> Vec<usize> {
        vec![event]
    }
}

struct NExecutor;

#[async_trait]
impl Executor<usize> for NExecutor {
    async fn execute(&self, _: usize) -> anyhow::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut engine: Engine<usize, usize> = Engine::default()
        .with_event_channel_capacity(SIZE)
        .with_action_channel_capacity(SIZE);

    let now = tokio::time::Instant::now();

    engine.add_collector(Box::new(NCollector));
    engine.add_strategy(Box::new(NStrategy));
    engine.add_executor(Box::new(NExecutor));

    if let Ok(mut set) = engine.run().await {
        while let Some(_) = set.join_next().await {}
    }

    println!("{}", now.elapsed().as_secs_f64());

    ()
}