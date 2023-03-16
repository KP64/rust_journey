use builder_pattern::{HttpMethod, Request};

fn main() {
    // * <Url, NoMethod, NotSealed>
    let builder_1 = Request::new().url("XD");
    // * <Url, Method, Sealed>
    let builder_2 = builder_1.clone().method(HttpMethod::GET).body("b").seal();

    // ! handle_2 is sealed
    // let handle_3 = builder_2.header("name", "value");

    let req = builder_1.clone().method(HttpMethod::POST).build();
    dbg!(builder_1, builder_2, req);
}
