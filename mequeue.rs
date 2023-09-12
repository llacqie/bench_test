use mequeue::Step;

enum Message {
    Event(u64),
    Action(Vec<u64>),
}

#[tokio::main]
async fn main() {
    let step1 = |e| async move {
        match e {
            Message::Event(e) => Some(Message::Action(vec![e])),
            _ => panic!(),
        }
    };
    let step2 = |_| async move { None };

    let now = tokio::time::Instant::now();

    let executor = mequeue::Root::new(|_| async { None }).map(step2).map(step1);

    for e in 0..10000000u64 {
        executor.enqueue(Message::Event(e)).await;
    }

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
