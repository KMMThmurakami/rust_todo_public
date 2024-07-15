macro_rules! sum {
    ( $ ($x:expr),* ) => {
        {
            let mut result = 0;
            $(
                result = result + $x;
            )*
            result
        }
    };
}

fn main() {
    print!("{}", sum![1, 2, 3, 4, 5]);
}
