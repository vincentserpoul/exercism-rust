pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut sad: Vec<(usize, usize)> = Vec::new();
    let mat: MatrixStats = MatrixStats::new(input);
    println!(
        "{:?} {:?} {:?} {:?}",
        mat.num_lines, mat.num_cols, mat.max_lines, mat.min_cols,
    );
    for x in 0..mat.num_cols {
        for y in 0..mat.num_lines {
            // Biggest in the line?
            if input[y][x] == mat.min_cols[x] && input[y][x] == mat.max_lines[y] {
                sad.push((y, x))
            }
        }
    }

    sad
}

struct MatrixStats {
    num_cols: usize,
    num_lines: usize,
    min_cols: Vec<u64>,
    max_lines: Vec<u64>,
}

impl MatrixStats {
    fn new(input: &[Vec<u64>]) -> MatrixStats {
        let nc: usize = input[0].len();
        let nl: usize = input.len();
        // max lines
        let mut ml: Vec<u64> = Vec::new();
        for line in input.iter() {
            if line.len() > 0 {
                ml.push(*line.iter().max().unwrap());
            }
        }
        // min cols
        let mut mc: Vec<u64> = Vec::new();
        for x in 0..nc {
            let mut min_col: u64 = input[0][x];
            for y in 0..nl {
                if input[y][x] <= min_col {
                    min_col = input[y][x];
                }
            }
            mc.push(min_col);
        }
        MatrixStats {
            num_cols: nc,
            num_lines: nl,
            min_cols: mc,
            max_lines: ml,
        }
    }
}
