use functional::pipe;

fn abs(i: i32) -> i32 {
    i.abs()
}

fn add(x: i32) -> impl FnOnce(i32) -> i32 {
    move |y| x + y
}

fn main() {
    pipe!{
        // let x = (add(2))((abs)(-4));
        let x = -4 |> abs |> add(2);
        println!("{}", x);
    }
}
