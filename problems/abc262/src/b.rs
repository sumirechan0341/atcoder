use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [[usize; 2]; m]
    };
    let mut adj_mat = vec![];
    // 0埋め
    for _ in 0..n {
        let mut v = vec![];
        for _ in 0..n {
            v.push(0);
        }
        adj_mat.push(v);
    }
    for uv in uvm {
        let u = uv[0];
        let v = uv[1];
        adj_mat[u-1][v-1] = 1;
        adj_mat[v-1][u-1] = 1;
    }
    let a2 = mat_product(&adj_mat, &adj_mat, n);
    let a3 = mat_product(&a2, &adj_mat, n);
    let mut tr = 0;
    for i in 0..n {
        tr += a3[i][i];
    }
    
    println!("{}", tr / 6);
}

fn mat_product(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for i in 0..n {
        let mut v = vec![];
        for j in 0..n {
            let mut sum = 0;
            for k in 0..n {
                sum += a[i][k] * b[k][j];
            }
            v.push(sum);
        }
        result.push(v);
    }
    return result;
}