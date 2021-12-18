// Module that deals with matrix multiplication and some rotation subroutines

type Vector<T> = [T; 3];
type Array<T> = [[T; 3]; 3];

pub fn print_vec() {

    let x: Vector::<f64> = [1., 3., 4.];

    for val in x.iter() {
        println!("{}", val);
    }
}

fn new_array<T>(arr: [T; 9]) -> Array<T>
    where T: Copy {
    [[arr[0], arr[1], arr[2]], [arr[3], arr[4], arr[5]], [arr[6], arr[7], arr[8]]] // Nothing has happened here, but later I will
                                                                                   // interpret this data in column-major order
}

// Ax = y
pub fn mat_times_vec(a: Array<f64>, x: Vector<f64>) -> Vector<f64> {

    [a[0][0] * x[0] +  a[1][0] * x[1] + a[2][0] * x[2],
     a[0][1] * x[0] +  a[1][1] * x[1] + a[2][1] * x[2],
     a[0][2] * x[0] +  a[1][2] * x[1] + a[2][2] * x[2]]

}

pub fn mat_mul(a: Array<f64>, b: Array<f64>) -> Array<f64> {

    [ mat_times_vec(a, b[0]), mat_times_vec(a, b[1]), mat_times_vec(a, b[2])]
}

pub fn print_array<T>(my_arr: Array<T>) where T: std::fmt::Display {

    println!("|{} {} {}|", my_arr[0][0], my_arr[1][0], my_arr[2][0]);
    println!("|{} {} {}|", my_arr[0][1], my_arr[1][1], my_arr[2][1]);
    println!("|{} {} {}|", my_arr[0][2], my_arr[1][2], my_arr[2][2]);
    println!("");
}

#[cfg(test)]
mod testing {

    #[test]
    fn print_array_test() {

        let my_arr = super::new_array::<f64>([1.0, 2., 3., 4., 5., 6., 7., 8., 9.]);
        let rhs = super::new_array::<f64>([2.0, 1.0, 0., 0., 4., 0., 0., 0., -2.,]);

        super::print_array::<f64>(my_arr);
        super::print_array::<f64>(rhs);

        let my_arr_id = super::mat_mul(my_arr, rhs);
        super::print_array(my_arr_id);

    }
}