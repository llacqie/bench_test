const SIZE: usize = 100000000usize;

enum Message {
    Event(usize),
    Action(Vec<usize>),
}

#[tokio::main]
async fn main() {
    let step1 = |e| async move {
        match e {
            Message::Event(e) => Some(("step2", Message::Action(vec![e]))),
            _ => panic!(),
        }
    };
    let step2 = |_| async move { None };

    let now = tokio::time::Instant::now();

    let executor = mequeuev1::Root::new(|_| async { None })
        .map("step1", step1)
        .map("step2", step2)
        .execute();

    for e in 0..SIZE {
        executor.enqueue("step1", Message::Event(e)).await;
    }

    println!("{}", now.elapsed().as_secs_f64());

    ()
}
