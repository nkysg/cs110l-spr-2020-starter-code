use crossbeam_channel;
use num_cpus;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    // TODO: implement parallel map!

    for _ in 0..input_vec.len() {
        output_vec.push(Default::default());
    }

    let (sender, receiver) = crossbeam_channel::unbounded();
    let (s, r) = crossbeam_channel::unbounded();
    let mut threads = Vec::new();
    let num_threads = num_cpus::get();

    for _ in 0..num_threads {
        let receiver = receiver.clone();
        let s = s.clone();
        threads.push(thread::spawn(move || {
            while let Ok((i, num)) = receiver.recv() {
                let res = f(num);
                s.send((i, res)).expect("send result error");
            }
            drop(s);
        }))
    }
    drop(s);

   
    for (i, item) in input_vec.into_iter().enumerate() {
        sender.send((i, item)).expect("send error");
    }

    drop(sender);
    while let Ok(res) = r.recv() {
        let (i, num) = res;
        output_vec[i] = num;
    }


    for handle in threads {
        handle.join().expect("inside error");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
