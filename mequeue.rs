use mequeue::executor::Executor;
use tokio::sync::{broadcast, mpsc};

const SIZE: usize = 100000000usize;

async fn checker(mut receiver: mpsc::Receiver<Vec<usize>>, ck: mpsc::Sender<()>) {
    while let Some(event) = receiver.recv().await {
        if event[0] == SIZE - 1 {
            ck.send(()).await.unwrap();
        };
    }
}

#[tokio::main]
async fn main() {
    let (ws, state) = broadcast::channel(512);
    let (we, event) = async_channel::bounded(512);

    let executor = Executor::new(state, event, 12);

    let (wx, receiver) = mpsc::channel(512);

    let worker = move |_, event: usize| {
        let wx = wx.clone();

        async move {
            wx.send(vec![event]).await.unwrap();
        }
    };

    let (ck, mut check) = mpsc::channel(512);

    tokio::spawn(executor.receive(worker));
    tokio::spawn(checker(receiver, ck));

    ws.send(()).unwrap();

    let now = tokio::time::Instant::now();

    for event in 0..SIZE {
        we.send(event).await.unwrap();
    }

    check.recv().await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
