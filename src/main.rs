use lists::first::List;

fn main() {
    let mut list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    loop {
        if let List::Cons(val, next) = list{
            list = *next;
            println!("{}", val);
        } else {
            break;
        }
    }
}
