use tokio::sync::mpsc;

const SIZE: usize = 10000000usize;

#[tokio::main]
async fn main() {
    let (ready, mut maybe_ready) = tokio::sync::mpsc::channel::<()>(1);

    let (event_sender, mut event_receiver) = mpsc::channel(SIZE);

    let executor = event_sender.clone();

    let inner = async move {
        while let Some(maybe_maybe_event) = event_receiver.recv().await {
            if let Some(maybe_event) = maybe_maybe_event {
                if let Some(event) = maybe_event {
                    if event == SIZE - 1 {
                        ready.send(()).await.unwrap();
                    };

                    event_sender.send(Some(None)).await.unwrap();
                } else {
                    event_sender.send(None).await.unwrap();
                }
            }
        }
    };

    tokio::spawn(inner);

    let now = tokio::time::Instant::now();

    for e in 0..SIZE {
        executor.send(Some(Some(e))).await.unwrap();
    }

    maybe_ready.recv().await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
