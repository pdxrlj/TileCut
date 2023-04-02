// use std::error::Error;
// use std::path::Path;
// use gdal::{Dataset as GdalDataset, DatasetOptions};
// use gdal::programs::raster::{BuildVRTOptions};
//
// use gdal::spatial_ref::{CoordTransform, SpatialRef};
// use crate::affine::Affine;
// use crate::bounds::Bounds;
//
// pub struct Dataset {
//     ds: GdalDataset,
// }
//
// impl Dataset {
//     pub fn open(path: &Path, disable_overviews: bool) -> Result<Dataset, Box<dyn Error>> {
//         let mut datasetOptions = DatasetOptions::default();
//         if disable_overviews {
//             datasetOptions.open_options = Some(&["OVERVIEW_LEVEL=NONE"]);
//         }
//         Ok(Dataset {
//             ds: GdalDataset::open_ex(path, datasetOptions)?,
//         })
//     }
//
//     pub fn bounds(&self) -> Result<Bounds, Box<dyn Error>> {
//         let (width, height) = self.ds.raster_size();
//         let transform = self.ds.geo_transform()?;
//         let affine = Affine::from_gdal(&transform);
//         Ok(Bounds {
//             xmin: affine.c,
//             ymin: affine.f + affine.e * height as f64,
//             xmax: affine.c + affine.a * width as f64,
//             ymax: affine.f,
//         })
//     }
//
//     pub fn transform_bounds(&self, crs: &SpatialRef) -> Result<Bounds, Box<dyn Error>> {
//         let bounds = self.bounds()?;
//         let src_crs = self.ds.spatial_ref()?;
//         let transform = CoordTransform::new(&src_crs, crs)?;
//
//         let out_bounds = transform.transform_bounds(&[bounds.xmin,
//             bounds.ymin,
//             bounds.xmax,
//             bounds.ymax], 21)?;
//
//         Ok(Bounds {
//             xmin: out_bounds[0],
//             ymin: out_bounds[1],
//             xmax: out_bounds[2],
//             ymax: out_bounds[3],
//         })
//     }
//
//     pub fn warped_vrt(&self, sp_ref: &SpatialRef) -> Result<Dataset, Box<dyn Error>> {
//         let mut vrt_options = BuildVRTOptions::new(["INIT_DEST", "NO_DATA", "NUM_THREADS", "1"])?;
//         let target_wkt = sp_ref.to_wkt()?;
//
//         let src_bounds = self.bounds()?;
//         // let vrt = build_vrt(Path::new("")?, src_bounds.into(), Some(vrt_options))?;
//     }
// }
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
