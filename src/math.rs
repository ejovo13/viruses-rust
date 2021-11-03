// Module that deals with matrix multiplication and some rotation subroutines

// use num::Num;

pub type Vec3 = [f64; 3];
pub type Mat3 = [Vec3; 3];


fn Mat3_zero() -> Mat3 {
    [ [0., 0., 0.], [0., 0., 0.], [0., 0., 0.] ]
}

fn Mat3_eye() -> Mat3 {
    [ [1., 0., 0.,], [0., 1., 0.,], [0., 0., 1.] ]
}

fn Mat3_set(A: &mut Mat3, i: usize, j: usize, val: f64) {
    A[i][j] = val;
}

fn Mat3_set_col(A:  &mut Mat3, icol: usize, c: Vec3) {
    for i in 0..3 {
        A[i][icol] = c[i];
    }
}

fn print_vec(x: &Vec3) {
    for val in x.iter() {
        println!("{}", val);
    }
}

fn print_row(x: &Vec3) {
    for val in x.iter() {
        print!("{} ", val);
    }
    println!("");
}

fn print_mat3(x: Mat3) {
    for val in x.iter() {
        print_row(val);
    }
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {

    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]

}

// Compute the dot product of two vectors
fn dot(a: Vec3, b: Vec3) -> f64 {

    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]

}

// Compute the hadamard product of two vectors
fn hadamard(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}

// Extract the column from a matrix
fn col(m: Mat3, c: usize) -> Vec3 {
    [m[0][c], m[1][c], m[2][c]]
}



pub fn rotate(v: Vec3, m: Mat3) -> Vec3 {

    // Rotate vector v by applying rotation matrix M
    [ dot(m[0], v), dot(m[1], v), dot(m[2], v) ]

    // Need to define matrix multiplication, knowing that M is row major order.

}

fn Vec3_add(u: Vec3, v: Vec3) -> Vec3 {
    [ u[0] + v[0], u[1] + v[1], u[2] + v[2] ]
}

fn Vec3_add_mut(u: &mut Vec3, v: Vec3) {
    u[0] += v[0];
    u[1] += v[1];
    u[2] += v[2];
}

fn Vec3_times_scalar(u: Vec3, k: f64) -> Vec3 {
    [ u[0] * k, u[1] * k, u[2] * k ]
}



// Multiple A*x
pub fn matvec(A: Mat3, x: Vec3) -> Vec3 {

    let mut res: Vec3 = [0.0, 0., 0.];

    for i in 0..3 {

        println!("Multiplying column {} with scalar {}", i, x[i]);

        println!("Col: {}", i);
        print_vec(&col(A,i));

        Vec3_add_mut(&mut res, Vec3_times_scalar(col(A,i), x[i]));
        print_vec(&res);
        println!("");

    }
    res
}


// Matrix multiplication of two Mat3'
pub fn matmul(m1: Mat3, m2: Mat3) -> Mat3 {

    let mut m = Mat3_zero();

    for i in 0..3 {
       Mat3_set_col(&mut m, i, matvec(m1, col(m2, i)));
    }

    m

}



#[cfg(test)]
mod tests {

    #[test]
    fn matmul_identity() {

        let mut A = super::Mat3_zero();
        let id = super::Mat3_eye();

        A[0] = [1., 2., 3.,];
        A[1] = [4., 5., 6.,];
        A[2] = [7., 8., 9.,];

        let prod = super::matmul(A, id);
        super::print_mat3(prod);

    }

    #[test]
    fn test_matvec() {

        let mut A = super::Mat3_zero();

        A[0] = [1., 2., 3.,];
        A[1] = [4., 5., 6.,];
        A[2] = [7., 8., 9.,];

        super::print_mat3(A);

        let v: super::Vec3 = [4., -3., 1.];

        super::print_vec(&v);

        let b = super::matvec(A, v);

        super::print_vec(&b);

    }

    #[test]
    fn test_print() {

        let mut A = super::Mat3_zero();
        super::Mat3_set(&mut A, 2, 1, 5.0);
        super::Mat3_set_col(&mut A, 0, [1., 2., 3.]);
        super::print_mat3(A);


        println!("Fail");

        super::print_row(&A[1]);
    }

    #[test]
    fn cross_product() {

        let x = [5., 14., 3.];
        let y = [2., 0., -2.];

        super::print_vec(&super::cross(x, y));
    }

    #[test]
    fn inspect_matrix() {
//                 ROW 1          ROW 2          ROW 3
        let m = [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];

        println!("m[0][0]: {}", m[0][0]);
        println!("m[1][0]: {}", m[1][0]);
        println!("m[2][0]: {}", m[2][0]);

    }

    #[test]
    fn rotate_identity() {

        let i = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];

        let v = [3., 4., 9.];

        println!("Rotating vector: ");
        super::print_vec(&v);

        println!("");
        super::print_vec(&super::rotate(v, i));

    }

    #[test]
    fn rotate_x() {

        let i = [[1., 0., 0.], [0., -1., 0.], [0., 0., 1.]];

        let v = [3., 4., 9.];

        println!("Rotating vector: ");
        super::print_vec(&v);

        println!("");
        super::print_vec(&super::rotate(v, i));

    }

}