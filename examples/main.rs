use ::std::{*,
};
use ::tokio::{
    prelude::{future, *},
};
use ::tokio_utils::*;

fn task (
    n: usize,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static
{
    future_match!(match n {
        | 1 => future::lazy(|| {
            println!("I got one!");
            future::ok(())
        }),
        | 2 => future::lazy(|| {
            println!("I got two!");
            future::ok(())
        }),
        | 3 => future::lazy(|| {
            println!("I got three!");
            future::ok(())
        }),
        | _ => future::lazy(|| {
            println!("I got three!");
            future::ok(())
        }),
    })
}

fn main ()
{
    task(1).wait().unwrap();
}
