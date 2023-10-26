fn pretty_print(matrix: &[&[i32]]) {
    for i in 0..3 {
        println!("[{} {} {}]", matrix[i][0], matrix[i][1], matrix[i][2]);
    }
}

fn transpose(matrix: &[&[i32]]) -> [[i32]] {
    let mut transposed_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed_matrix[i][j] = matrix[j][i];
        }
    }
    transposed_matrix
}
fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("matrix:");
    pretty_print(&matrix);
    let transposed = transpose(&matrix);
    println!("transpose:");
    pretty_print(&transposed);
}
