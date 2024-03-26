use std::fmt::Display;

fn main() {
    let matrix = [['q', 'r', 's'], ['t', 'u', 'v'], ['w', 'x', 'y']];
    let transposed_matrix = transpose(&matrix);
    println!(
        "{}\n{}",
        matrix_to_string(&matrix),
        matrix_to_string(&transposed_matrix)
    )
}
/// Swap rows and columns of matrix.
/// example of equivalent loops
///```rust
///    let mut row_i = 0;
///    let mut col_i = 0;
///
///    while row_i < W {
///        while col_i < H {
///
///            transpose[row_i][col_i] = matrix[col_i][row_i];
///
///            col_i += 1;
///        }
///
///        row_i += 1;
///    }
///
///    let mut row_i = 0;
///    let mut col_i = 0;
///
///    loop {
///        if !(row_i < W) {
///            break
///        }
///
///        loop {
///            if !(col_i < H) {
///                break;
///            }
///
///            transpose[row_i][col_i] = matrix[col_i][row_i];
///
///            col_i += 1;
///        }
///
///        row_i += 1;
///    }
/// ```
fn transpose<E: Default + Copy, const W: usize, const H: usize>(
    matrix: &[[E; W]; H],
) -> [[E; H]; W] {
    let mut transpose = [[E::default(); H]; W];

    for row_i in 0..W {
        for column_i in 0..H {
            // let element_i = (row_i, column_i);

            transpose[row_i][column_i] = matrix[column_i][row_i];
        }
    }

    return transpose;
}
fn matrix_to_string<E: Display, const W: usize, const H: usize>(matrix: &[[E; W]; H]) -> String {
    let mut output = String::new();

    for row in matrix.iter() {
        for element in row.iter() {
            output.push_str(&format!("{} ", element));
        }

        output.push('\n');
    }

    return output;
}

#[test]
fn to_string_test() {
    // Set up test
    let matrix = [[1], [2], [3]];
    let expected_output = "1 \n2 \n3 \n";

    // Perform text
    let output = matrix_to_string(&matrix);
    println!("{}", output);

    // Check test
    assert_eq!(expected_output, output)
}

#[test]
fn transpose_test() {
    let matrix = [[00, 01, 02], [10, 11, 12], [20, 21, 22]];
    let expected_output = [[00, 10, 20], [01, 11, 21], [02, 12, 22]];

    let transpose = transpose(&matrix);
    println!("original\n{:?}\ntranspose\n{:?}\n", matrix, transpose);

    assert!(transpose == expected_output)
}
