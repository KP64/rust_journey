mod iter_diff;
mod ordered_arr;
mod self_impl;
fn main() {
    self_impl::self_impl();
    println!();
    iter_diff::iter_diff();
    println!();
    ordered_arr::ordered_arr();
    println!();
}
