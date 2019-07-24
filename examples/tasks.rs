use ::std::{*,
    sync::Arc,
};
use ::tokio::{
    fs,
    prelude::{future, *},
    runtime::Runtime,
};
use ::tokio_utils::*;

fn unwrap<T> (err: impl fmt::Display) -> T
{
    panic!("Error: {}", err);
}

fn task1 (
    n: usize,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static
{
    let filename: Arc<path::Path> =
        path::PathBuf::from(format!("tmp_{}.temp", n)).into()
    ;

    future_chain! {
        let file =
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(Arc::clone(&filename))
        ;
        ::tokio::io::write_all(file, b"Hello, World!").map(mem::drop);
        let contents = fs::read(Arc::clone(&filename));
        future_branch!(match &contents[..] {
            | b"Hello, World!" => {
                ::tokio::timer::Delay::new(time::Instant::now() +
                    time::Duration::from_secs(1)
                ).map_err(unwrap)
            },
            | _ => future_unreachable(),
        });
        fs::remove_file(filename);
    }
}

fn non_async_task (n: usize) -> io::Result<()>
{
    use ::std::fs;

    let filename = format!("tmp_{}.temp", n);
    let file =
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&filename)?
    ;
    io::Write::write_all(&mut {file}, b"Hello, World!")?;
    let contents = fs::read(&filename)?;
    match &contents[..] {
        | b"Hello, World!" => {
            thread::sleep(time::Duration::from_secs(1));
        },
        | _ => unreachable!(),
    }
    fs::remove_file(&filename)?;
    Ok(())
}

fn task2 (
    n: usize,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static
{
    future::lazy(move || future::done(non_async_task(n)))
}

fn main ()
{
    let mut rt = Runtime::new().unwrap();
    (0 .. 50).for_each(|i| {
        rt.spawn(
            task2(i)
                .map_err(unwrap)
        );
    });
    rt.shutdown_on_idle().wait().unwrap();
}
