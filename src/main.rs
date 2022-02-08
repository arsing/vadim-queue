#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]

use vadim_queue::{Queue as _, QueueImpl, Reader as _, WakerStrategyImpl};

use std::sync::Arc;
use std::task::Poll;

use futures_util::{StreamExt as _, TryStreamExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();

    let a_queue = Arc::new(QueueImpl::<u64, WakerStrategyImpl>::new());
    let b_queue = Arc::new(QueueImpl::<u64, WakerStrategyImpl>::new());
    let c_queue = Arc::new(QueueImpl::<u64, WakerStrategyImpl>::new());

    let tasks = futures_util::stream::FuturesUnordered::new();

    tasks.push(tokio::task::spawn({
        let a_queue = a_queue.clone();
        async move {
            let mut i = 1000;
            loop {
                #[cfg(debug_assertions)]
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

                a_queue.append(i);
                i += 1;

                #[cfg(not(debug_assertions))]
                if i % 1000 == 0 {
                    tokio::task::yield_now().await;
                }
            }
        }
    }));

    tasks.push(tokio::task::spawn({
        let b_queue = b_queue.clone();
        async move {
            let mut i = 2000;
            loop {
                #[cfg(debug_assertions)]
                tokio::time::sleep(std::time::Duration::from_millis(1234)).await;

                b_queue.append(i);
                i += 1;

                #[cfg(not(debug_assertions))]
                if i % 1000 == 0 {
                    tokio::task::yield_now().await;
                }
            }
        }
    }));

    tasks.push(tokio::task::spawn({
        let c_queue = c_queue.clone();
        async move {
            let mut i = 3000;
            loop {
                #[cfg(debug_assertions)]
                tokio::time::sleep(std::time::Duration::from_millis(2345)).await;

                c_queue.append(i);
                i += 1;

                #[cfg(not(debug_assertions))]
                if i % 1000 == 0 {
                    tokio::task::yield_now().await;
                }
            }
        }
    }));

    tasks.push(tokio::task::spawn(async move {
        let mut a_reader = a_queue.reader();
        let mut i_prev = None;

        while let Some(i) = a_reader.next().await {
            if cfg!(debug_assertions) || i % 100_000 == 0 {
                println!("[{:07.3}] a: {i:>9}", start_time.elapsed().as_secs_f32());
            }

            if let Some(i_prev_) = &mut i_prev {
                assert_eq!(i, *i_prev_ + 1, "a got unexpected value");
                *i_prev_ = i;
            }
            else {
                i_prev = Some(i);
            }
        }
    }));

    tasks.push(tokio::task::spawn(async move {
        let mut b_reader = b_queue.reader();
        let mut i_prev = None;

        while let Some(i) = b_reader.next().await {
            if cfg!(debug_assertions) || i % 100_000 == 0 {
                println!("[{:07.3}] b:           {i:>9}", start_time.elapsed().as_secs_f32());
            }

            if let Some(i_prev_) = &mut i_prev {
                assert_eq!(i, *i_prev_ + 1, "b got unexpected value");
                *i_prev_ = i;
            }
            else {
                i_prev = Some(i);
            }
        }
    }));

    tasks.push(tokio::task::spawn({
        let c_queue = c_queue.clone();
        let mut i_prev = None;

        async move {
            let mut c_reader = c_queue.reader();
            while let Some(i) = c_reader.next().await {
                if cfg!(debug_assertions) || i % 100_000 == 0 {
                    println!("[{:07.3}] c:                     {i:>9}", start_time.elapsed().as_secs_f32());
                }

                if let Some(i_prev_) = &mut i_prev {
                    assert_eq!(i, *i_prev_ + 1, "c got unexpected value");
                    *i_prev_ = i;
                }
                else {
                    i_prev = Some(i);
                }
            }
        }
    }));

    tasks.push(tokio::task::spawn(async move {
        let bench = std::env::var_os("BENCH").is_some();

        // Same queue as c_reader, but (intentionally) includes delays to demonstrate catching-up behavior.

        tokio::time::sleep(std::time::Duration::from_millis(10000)).await;

        let mut d_reader = c_queue.reader();
        let mut i_prev = None;

        loop {
            while let Poll::Ready(i) = d_reader.read() {
                if cfg!(debug_assertions) || i % 100_000 == 0 {
                    println!("[{:07.3}] d:                               {i:>9} (s)", start_time.elapsed().as_secs_f32());
                    if bench {
                        std::process::exit(0);
                    }
                }

                if let Some(i_prev_) = &mut i_prev {
                    assert_eq!(i, *i_prev_ + 1, "d got unexpected value");
                    *i_prev_ = i;
                }
                else {
                    i_prev = Some(i);
                }
            }

            let i = d_reader.next().await.unwrap();
            if cfg!(debug_assertions) || i % 100_000 == 0 {
                println!("[{:07.3}] d:                               {i:>9} (a)", start_time.elapsed().as_secs_f32());
            }

            if let Some(i_prev_) = &mut i_prev {
                assert_eq!(i, *i_prev_ + 1, "d got unexpected value");
                *i_prev_ = i;
            }
            else {
                i_prev = Some(i);
            }

            tokio::time::sleep(std::time::Duration::from_millis(10000)).await;
        }
    }));

    tasks.try_for_each(|()| async { Ok(()) }).await?;
    Ok(())
}
