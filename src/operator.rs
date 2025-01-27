use pgrx::prelude::*;

/// Square Euclidean distance. Try this one if you don't have any special requirements.
#[pg_operator(immutable, parallel_safe)]
#[opname(<->)]
fn square_euclidean_distance(left: Vec<f32>, right: Vec<f32>) -> f32 {
    if left.len() != right.len() {
        error!(
            "wrong dimension: left({}) != right({})",
            left.len(),
            right.len()
        );
    }
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum()
}

/// Dot product distance.
#[pg_operator(immutable, parallel_safe)]
#[opname(<#>)]
fn dot_product_distance(left: Vec<f32>, right: Vec<f32>) -> f32 {
    if left.len() != right.len() {
        error!(
            "wrong dimension: left({}) != right({})",
            left.len(),
            right.len()
        );
    }
    left.iter().zip(right.iter()).map(|(x, y)| x * y).sum()
}

/// Cosine distance. Similar to Euclidean distance but with a normalization.
/// Use this if your vectors are not normalized.
#[pg_operator(immutable, parallel_safe)]
#[opname(<=>)]
fn cosine_distance(left: Vec<f32>, right: Vec<f32>) -> f32 {
    if left.len() != right.len() {
        error!(
            "wrong dimension: left({}) != right({})",
            left.len(),
            right.len()
        );
    }
    let dot_product: f32 = left.iter().zip(right.iter()).map(|(x, y)| x * y).sum();
    let norm_left = left.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm_right = right.iter().map(|y| y.powi(2)).sum::<f32>().sqrt();
    dot_product / (norm_left * norm_right)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_square_euclidean_distance() {
        let distance = Spi::get_one::<f32>("SELECT ARRAY[0, 1] <-> ARRAY[3, 2]");
        assert_eq!(distance, Ok(Some(10.0)));
    }

    #[pg_test]
    fn test_dot_product_distance() {
        let distance = Spi::get_one::<f32>("SELECT ARRAY[5, 1] <#> ARRAY[1, 2]");
        assert_eq!(distance, Ok(Some(7.0)));
    }

    #[pg_test]
    fn test_cosine_distance() {
        let distance = Spi::get_one::<f32>("SELECT ARRAY[4, 4] <#> ARRAY[2, 2]");
        assert_eq!(distance, Ok(Some(16.0)));
    }
}
