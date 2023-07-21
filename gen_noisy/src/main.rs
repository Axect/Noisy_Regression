use peroxide::fuga::*;

fn main() {
    let noise = Normal(0., 0.5);
    let p_true = vec![20f64, 10f64, 1f64, 50f64];
    
    let domain = linspace(0, 100, 1000);
    let y = f(&domain, &p_true).add_v(&noise.sample(1000));
    
    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(domain));
    df.push("y", Series::new(y));
    df.print();

    df.write_parquet("../data.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet file");
}

fn f(domain: &Vec<f64>, p: &[f64]) -> Vec<f64> {
    domain.fmap(|x| p[0] * (-x / p[1]).exp() + p[2] * x * (-x / p[3]).exp())
}
