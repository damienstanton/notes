// Memoized version
pub fn max_revenue_memoize(len: i32, sizes: &Vec<i32>, prices: &Vec<f64>) -> f64 {
    let n = len as usize;
    let k = sizes.len();
    assert_eq!(prices.len(), k);

    let mut table = vec![0.; n + 1];
    let mut vals = vec![];

    for i in 1..n + 1 {
        for j in 0..k {
            let i = i as i32;
            if i - sizes[j] >= 0 {
                let idx = (i - sizes[j]) as usize;
                vals.push(prices[j] + table[idx] as f64);
                vals.push(0.);
                table[i as usize] = vals.clone().into_iter().reduce(f64::max).unwrap()
            }
        }
    }
    table[n]
}

#[allow(unused)]
pub fn max_revenue_with_soln(len: i32, sizes: &Vec<i32>, prices: &Vec<f64>) -> f64 {
    let n = len as usize;
    let k = sizes.len();
    assert_eq!(prices.len(), k);

    let mut table = vec![0.; n + 1];
    let mut solns = vec![-1i32, len + 1];

    for i in 1..n + 1 {
        table[i] = 0.;
        let mut options_with_solns = vec![];
        for j in 0..k {
            if i as i32 - sizes[j] >= 0 {
                let tup = (prices[j] + table[i - sizes[j] as usize], i as i32);
                options_with_solns.push(tup)
            }
            options_with_solns.push((0., -1));
        }
    }

    // let mut cuts = vec![];

    todo!("check max comparisons for tuples in Rust")
}

#[test]
fn examples() {
    let sizes = vec![1, 3, 5, 10, 30, 50, 75];
    let prices = vec![0.1, 0.2, 0.4, 0.9, 3.1, 5.1, 8.2];
    assert_eq!(3.0, max_revenue_memoize(30, &sizes, &prices).round());
    assert_eq!(5.0, max_revenue_memoize(50, &sizes, &prices).round());
    assert_eq!(33.0, max_revenue_memoize(300, &sizes, &prices).round());
}
