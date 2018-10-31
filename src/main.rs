#[macro_use]
extern crate warp;

use warp::Filter;

fn main() {
    let default_route = warp::any().map(|| "Hello, World!");
    let name_route = warp::path!("hello" / String).map(|n| format!("Hello, {}", n));

    let routes = name_route.or(default_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
