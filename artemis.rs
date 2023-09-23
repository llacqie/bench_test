use artemis_core::{
    engine::Engine,
    types::{Collector, CollectorStream, Executor, Strategy},
};
use async_trait::async_trait;
use tokio::sync::mpsc;

const SIZE: usize = 100000000usize;

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

struct NExecutor(mpsc::Sender<()>);

#[async_trait]
impl Executor<usize> for NExecutor {
    async fn execute(&self, a: usize) -> anyhow::Result<()> {
        if a == SIZE - 1 {
            self.0.send(()).await.unwrap();
        };

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut engine: Engine<usize, usize> = Engine::default()
        .with_event_channel_capacity(SIZE)
        .with_action_channel_capacity(SIZE);

    let (ready, mut maybe_ready) = tokio::sync::mpsc::channel::<()>(1);

    engine.add_collector(Box::new(NCollector));
    engine.add_strategy(Box::new(NStrategy));
    engine.add_executor(Box::new(NExecutor(ready)));

    let now = tokio::time::Instant::now();

    if let Ok(mut set) = engine.run().await {
        while let Some(_) = set.join_next().await {}
    }

    maybe_ready.recv().await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
