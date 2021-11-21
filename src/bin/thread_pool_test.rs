use rust_ml::threading::thread_pool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(10).unwrap();

    for i in 0..100 {
        pool.execute(move || {
             let _o = i + i;
        })
    }
}