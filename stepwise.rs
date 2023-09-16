use stepwise::Executor;

const SIZE: usize = 10000000usize;

enum Message {
    Event(usize),
    Action(Vec<usize>),
}

#[tokio::main]
async fn main() {
    let step1 = |e| async move {
        match e {
            Message::Event(e) => Some(Message::Action(vec![e])),
            _ => panic!(),
        }
    };
    let (ready, mut maybe_ready) = tokio::sync::mpsc::channel::<()>(1);

    let step2 = |a| {
        let ready = ready.clone();

        async move {
            match a {
                Message::Action(a) => {
                    if a[0] == SIZE - 1 {
                        ready.send(()).await.unwrap();
                    };
                    None
                }
                _ => panic!(),
            }
        }
    };

    let now = tokio::time::Instant::now();

    let executor = stepwise::new(step1).map(step2);

    for e in 0..SIZE {
        executor.execute(Message::Event(e)).await;
    }

    maybe_ready.recv().await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
