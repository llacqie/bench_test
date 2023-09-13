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
    let step2 = |_| async move { None };

    let now = tokio::time::Instant::now();

    let executor = stepwise::new(step1).map(step2);

    for e in 0..SIZE {
        executor.execute(Message::Event(e)).await;
    }

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
