fn main() {
    println!("enter an expression in reversed notation: ");

    type Num = f64;

    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("an error occurs");

    let mut num_stack: Vec<Num> = Vec::with_capacity(2);
    let expression = input.trim_end().split(' ');

    for element in expression {
        match element {
            "*" | "+" | "-" | "/" => {
                let a = num_stack.pop().unwrap();
                let b = num_stack.pop().unwrap();

                let r = match element {
                    "*" => a * b,
                    "+" => a + b,
                    "-" => a - b,
                    "/" => a / b,
                    _ => todo!(),
                };

                num_stack.push(r)
            }
            s => num_stack.push(s.parse().expect("an error ocuurs while parsing a number")),
        }
    }

    println!("{:?}", num_stack);
}
