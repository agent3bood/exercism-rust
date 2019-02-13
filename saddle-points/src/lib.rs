pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (i, row) in input.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let mut is_saddle = true;
            for (k, k_row) in input.iter().enumerate() {
                for (l, l_item) in k_row.iter().enumerate() {
                    if j == l && item > l_item {
                        is_saddle = false;
                    }
                    if i == k && item < l_item {
                        is_saddle = false;
                    }
                }
            }
            if is_saddle {
                res.push((i, j));
            }
        }
    }
    res
}
