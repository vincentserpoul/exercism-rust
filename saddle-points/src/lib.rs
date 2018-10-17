pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }

    // Calculate max lines
    let max_lines = input
        .iter()
        .map(|line| *line.iter().max().unwrap_or(&0))
        .collect::<Vec<u64>>();

    // println!("max in line {:?}", max_lines);

    // Calculate min cols
    let min_cols = (0..input[0].len())
        .map(|col| input.iter().map(|ref line| line[col]).min().unwrap_or(0))
        .collect::<Vec<u64>>();

    // println!("min in column {:?}", min_cols);

    // println!(
    //     "enum {:?}",
    //     input.iter().enumerate().map(|line| line
    //         .1
    //         .iter()
    //         .enumerate()
    //         .filter(|col| min_cols[col.0] == *col.1 && max_lines[line.0] == *col.1)
    //         .map(move |col| (col.0, line.0)))
    // );

    let mut sad: Vec<(usize, usize)> = Vec::new();
    for y in 0..input.len() {
        for (x, _v) in min_cols.iter().enumerate().take(input[0].len()) {
            // Biggest in the line?
            if input[y][x] == min_cols[x] && input[y][x] == max_lines[y] {
                sad.push((y, x))
            }
        }
    }

    sad
}

// Maybe one day, I can play with lifetimes and ownership within iterators
// println!(
//     "enum {:?}",
//     input.iter().enumerate().map(|line| line
//         .1
//         .iter()
//         .enumerate()
//         .filter(|col| min_cols[col.0] == *col.1 && max_lines[line.0] == *col.1)
//         .map(move |col| (col.0, line.0)))
// );
