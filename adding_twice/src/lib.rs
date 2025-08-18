pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

pub fn twice<F>(f: F) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn(i32) -> i32 + 'static,
{
    Box::new(move |x| f(f(x)))
}