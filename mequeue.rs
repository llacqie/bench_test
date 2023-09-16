const SIZE: usize = 10000000usize;

type Ref<T1> = std::sync::Arc<T1>;

enum Message {
    Event(usize),
    Action(Vec<usize>),
}

#[tokio::main]
async fn main() {
    let (ready, mut maybe_ready) = tokio::sync::mpsc::channel::<()>(1);

    let event_dispatcher = Ref::new(move |event| {
        let ready = ready.clone();

        async move {
            match event {
                Message::Event(e) => Some(Message::Action(vec![e])),
                Message::Action(a) => {
                    if a[0] == SIZE - 1 {
                        ready.send(()).await.unwrap();
                    };
                    None
                }
            }
        }
    });

    let await_dispatcher = Ref::new(|_| async move {});

    let now = tokio::time::Instant::now();

    let (executor, _) = mequeue::execute(SIZE, event_dispatcher, await_dispatcher);

    for e in 0..SIZE {
        executor.send(Some(Message::Event(e))).await.unwrap();
    }

    maybe_ready.recv().await.unwrap();

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
