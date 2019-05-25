use std::io;

fn main() {

    loop {
        println!("Please input your number: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");
            
        let number: u32 = number.trim().parse()
            .expect("Please type a number!");

        let _result = fibo(number);
        println!("result={}", _result);
    }
}


fn fibo(_input:u32) -> u32 {
    
    let mut _prev_sum = 1;
    let mut _sum = 1;
    
    if _input == 0 {
        return 0;
    }
   
    for _i in 2.._input {
        let _tmp = _prev_sum;
        _prev_sum = _sum;
        _sum += _tmp;
    }
    
    _sum
}
