use gdal::raster::ResampleAlg;
use gdal::Dataset;
use std::f64::consts::PI;
use std::path::PathBuf;

#[warn(unused_variables)]
pub struct Utils {
    pub data_set: Dataset,
    pub initial_resolution: f64,
}

#[warn(unused_variables)]
impl Utils {
    pub fn new(path: PathBuf) -> Self {
        let data_set = Dataset::open(path)
            .map_err(|e| println!("err:{}", e))
            .unwrap();
        Utils {
            data_set,
            initial_resolution: f64::from(2) * PI * 6378137.0 / 256.0,
        }
    }
    pub fn resolution(&self, z: i64) -> f64 {
        let zpow = 2_i64.pow(z as u32);
        self.initial_resolution / zpow as f64
    }

    pub fn resampling_raster(&self, z: i64, path: PathBuf) {
        let driver = self.data_set.driver();
        let resolution = self.resolution(z);
        println!("重采样后的分辨率为：{}", resolution);
        let geo_transform = self.data_set.geo_transform().unwrap();
        println!(
            "geo_transform[1]:{} geo_transform[5]:{}",
            geo_transform[1], geo_transform[5]
        );

        let (x, y) = self.scale_zoom_size(
            self.data_set.raster_size().0,
            self.data_set.raster_size().1,
            geo_transform[1],
            geo_transform[5],
            resolution,
        );
        println!("重采样后的大小为：{}*{}", x, y);

        let mut outPut = driver
            .create(
                path,
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                self.data_set.rasterband(1).unwrap().band_type() as isize,
            )
            .unwrap();

        let [x, _, xr, yx, yr, _] = self.data_set.geo_transform().unwrap();

        let geo_transform: [f64; 6] = [x, resolution, xr, yx, yr, -resolution];

        outPut.set_geo_transform(&geo_transform).unwrap();
        outPut
            .set_projection(self.data_set.projection().as_str())
            .unwrap();

        gdal::Driver:: (
            &self.data_set,
            &mut outPut,
            None,
            None,
            ResampleAlg::NearestNeighbour,
            None,
        );

        // let _ = gdal::raster::rasterize(
        //     &mut outPut,
        //     &self.data_set.raster_count(),
        //     Some(&options),
        //     ResampleAlg::NearestNeighbour,

        // ).unwrap();
    }

    pub fn scale_zoom_size(
        &self,
        width: usize,
        height: usize,
        x_size: f64,
        y_size: f64,
        resolution: f64,
    ) -> (usize, usize) {
        println!(
            "width:{} height:{} x_size:{} y_size:{} resolution:{}",
            width, height, x_size, y_size, resolution
        );
        let x = (width as f64 * (x_size / resolution)).round() as usize;
        let y = (height as f64 * (y_size / resolution)).round().abs() as usize;
        return (x, y);
    }
}

#[cfg(test)]
mod utils_test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn resampling_raster_test() {
        let path = PathBuf::from("converted.tif");
        let utils = Utils::new(path);
        utils.resampling_raster(18, PathBuf::from("3.tif"));
    }
}
