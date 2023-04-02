#[derive(Debug)]
pub struct Affine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Affine {
    pub fn from_gdal(transform: &[f64; 6]) -> Affine {
        Self {
            a: transform[1],
            b: transform[2],
            c: transform[0],
            d: transform[4],
            e: transform[5],
            f: transform[3],
        }
    }
}

