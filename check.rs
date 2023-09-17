use tokio::sync::mpsc;

const SIZE: usize = 10000000usize;

enum Step<T1> {
    S1(T1),
    S2(T1),
}

#[tokio::main]
async fn main() {
    let (event_sender, mut event_receiver) = mpsc::channel(SIZE);

    let channel = event_sender.clone();

    let inner = async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                Step::S1(e) => {
                    event_sender.send(Step::S2(e)).await.unwrap();
                }
                Step::S2(e) => {
                    if e == SIZE - 1 {
                        break;
                    };
                }
            }
        }
    };

    let jh = tokio::spawn(inner);

    let now = tokio::time::Instant::now();

    for e in 0..SIZE {
        channel.send(Step::S1(e)).await.unwrap();
    }

    jh.await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
