// Module that deals with matrix multiplication and some rotation subroutines

type vector<T> = [T; 3];


pub fn print_vec() {

    let x: vector::<f64> = [1., 3., 4.];

    for val in x.iter() {
        println!("{}", val);
    }


}