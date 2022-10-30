mod iter_diff;
mod self_impl;
mod ordered_arr;
fn main() {
    self_impl::self_impl();
    println!();
    iter_diff::iter_diff();
    println!();
    ordered_arr::ordered_arr();
    println!();
}
