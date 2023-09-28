fn step(x: usize) -> usize {
    if x < 10 {
        return x.max(1);
    }
    let rest = x % 10;
    return rest.max(1) * step((x-rest)/10)
}

fn get_final(x: usize) -> usize {
    let mut last_number = x;
    loop {
        let result = step(last_number);
        if result == last_number {
            break
        }
        last_number = result;
    }
    last_number
}

pub async fn main() {
    let mut threads = Vec::new();
    for i in 0..100_000 {
        threads.push(tokio::spawn(async move {
            get_final(i)
        }))
    }
    let mut results = Vec::new();
    for thread in threads {
        results.push(thread.await.unwrap());
    }
    dbg!(results);
}
