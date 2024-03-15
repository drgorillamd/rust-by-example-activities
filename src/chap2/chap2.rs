use std::fmt::{self, Formatter, Display};

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(input: Matrix) -> (Matrix) {
    Matrix(input.0, input.2, input.1, input.3)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Implement the `Display` trait for `Matrix`.
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {0} {1} )\n( {2} {3} )", self.0, self.1, self.2, self.3)
    }
}

pub fn display_matrix() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
}

pub fn display_transpose() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));    
}