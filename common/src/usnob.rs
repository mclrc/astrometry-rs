/// USNO-B Catalog File (.cat) Reader
/// Ported from the relevant Astrometry.net C code
use std::{
    fs::File,
    io::{BufReader, Read},
    ops::Deref,
    path::Path,
    slice,
};

use anyhow::Result;

use crate::error::AstroError;

use serde::Serialize;

const USNOB_RECORD_SIZE: usize = 80;

#[inline]
#[allow(clippy::excessive_precision)]
fn arcsec_to_degrees(arcsec: f64) -> f64 {
    arcsec * 0.00027777777777777778
}

#[derive(Debug, Serialize)]
pub struct Observation {
    // 0 to 99.99
    pub mag: f32,

    // Field number in the original survey. 1-937
    pub field: i16,

    // The original survey.
    pub survey: u32,

    // star/galaxy estimate.  0=galaxy, 11=star. 19=no value computed.
    // (but note, in fact values 12, 13, 14, 15 and possibly others exist
    //  in the data files as well!)
    pub star_galaxy: u8,

    // [degrees]
    pub xi_resid: f32,

    // [degrees]
    pub eta_resid: f32,

    // source of photometric calibration:
    //  0=bright photometric standard on this plate
    //  1=faint pm standard on this plate
    //  2=faint " " one plate away
    //  etc
    pub calibration: u8,

    // back-pointer to PMM file.
    pub pmmscan: i32,
}

#[derive(Debug, Serialize)]
pub struct USNOBObject {
    // Identifier used internally, not part of the USNO-B files
    pub usnob_id: String,

    // Right Ascension in degrees
    pub ra: f64,
    // Declination in degrees
    pub dec: f64,

    // Uncertainty in Right Ascension in degrees
    pub sigma_ra: f32,
    // Uncertainty in Declination in degrees
    pub sigma_dec: f32,

    // Fit uncertainty in Right Ascension in degrees
    pub sigma_ra_fit: f32,
    // Fit uncertainty in Declination in degrees
    pub sigma_dec_fit: f32,

    // Proper motion in Right Ascension in arcsec/yr
    pub pm_ra: f32,
    // Proper motion in Declination in arcsec/yr
    pub pm_dec: f32,

    // Uncertainty in proper motion in Right Ascension in arcsec/yr
    pub sigma_pm_ra: f32,
    // Uncertainty in proper motion in Declination in arcsec/yr
    pub sigma_pm_dec: f32,

    // Motion probability
    pub pm_prob: f32,

    // Epoch year, range from 1950 to 2050
    pub epoch: f32,

    // Number of detections; different meanings based on the value
    pub n_detections: u8,

    // Flags for diffraction spike, motion catalog, and YS4.0 correlation
    pub diffraction_spike: bool,
    pub motion_catalog: bool,
    pub ys4: bool,

    // Observations for this object, stored in a fixed order
    pub observations: [Option<Observation>; 5],
}

fn extract_digit_chunks<const N: usize>(mut n: u32, chunks: [usize; N]) -> [u32; N] {
    let mut nums = [0; N];

    for (idx, &chunk) in chunks.iter().enumerate() {
        nums[idx] = n % 10u32.pow(chunk as u32);
        n /= 10u32.pow(chunk as u32);
    }

    nums
}

macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            Err(AstroError::new(&format!($($arg)*)))?;
        }
    };
}

impl USNOBObject {
    fn from_bytes(buffer: &[u8; USNOB_RECORD_SIZE], id: usize) -> Result<Self> {
        let uline =
            unsafe { slice::from_raw_parts(buffer.as_ptr() as *const u32, USNOB_RECORD_SIZE / 4) };

        let ra = arcsec_to_degrees(uline[0] as f64 * 0.01);
        ensure!((0.0..360.0).contains(&ra), "RA {} out of range", ra);

        let dec = arcsec_to_degrees(uline[1] as f64 * 0.01) - 90.0;
        ensure!((-90.0..=90.0).contains(&dec), "DEC {} out of range", dec);

        let [pm_ra, pm_dec, pm_prob, motion_catalog] = extract_digit_chunks(uline[2], [4, 4, 1, 1]);

        let pm_ra = 0.002 * (pm_ra as f64 - 5000.0);
        let pm_dec = 0.002 * (pm_dec as f64 - 5000.0);
        let pm_prob = 0.1 * pm_prob as f64;
        let motion_catalog = motion_catalog == 1;

        let [sigma_pm_ra, sigma_pm_dec, sigma_ra_fit, sigma_dec_fit, n_detections, diffraction_spike] =
            extract_digit_chunks(uline[3], [3, 3, 1, 1, 1, 1]);

        let sigma_pm_ra = 0.001 * sigma_pm_ra as f64;
        let sigma_pm_dec = 0.001 * sigma_pm_dec as f64;
        let sigma_ra_fit = arcsec_to_degrees(0.1 * sigma_ra_fit as f64);
        let sigma_dec_fit = arcsec_to_degrees(0.1 * sigma_dec_fit as f64);
        let n_detections = n_detections as u8;
        let diffraction_spike = diffraction_spike == 1;

        let [sigma_ra, sigma_dec, epoch, ys4] = extract_digit_chunks(uline[4], [3, 3, 3, 1]);

        let sigma_ra = arcsec_to_degrees(0.001 * sigma_ra as f64);
        let sigma_dec = arcsec_to_degrees(0.001 * sigma_dec as f64);
        let epoch = 1950.0 + 0.01 * epoch as f64;
        let ys4 = ys4 == 1;

        let observations = (0..5)
            .map(|obs_idx| {
                let [mag, field, survey, star_galaxy] =
                    extract_digit_chunks(uline[5 + obs_idx], [4, 3, 1, 2]);

                if field == 0 {
                    return None;
                }

                let mag = 0.01 * mag as f32;

                let [xi_resid, eta_resid, calibration] =
                    extract_digit_chunks(uline[10 + obs_idx], [4, 4, 1]);

                let pmmscan = uline[15 + obs_idx] as i32;

                Some(Observation {
                    mag,
                    field: field as i16,
                    survey,
                    star_galaxy: star_galaxy as u8,
                    xi_resid: if n_detections >= 2 && field == 0 {
                        arcsec_to_degrees(0.01 * xi_resid as f64) as f32
                    } else {
                        0.0
                    },
                    eta_resid: if n_detections >= 2 && field == 0 {
                        arcsec_to_degrees(0.01 * eta_resid as f64) as f32
                    } else {
                        0.0
                    },
                    calibration: calibration as u8,
                    pmmscan,
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let slice = ((dec + 90.0) * 10.0).floor();
        let id = format!("{:04}-{:07}", slice, id);

        Ok(USNOBObject {
            usnob_id: id.to_string(),
            ra,
            dec,
            sigma_ra: sigma_ra as f32,
            sigma_dec: sigma_dec as f32,
            sigma_ra_fit: sigma_ra_fit as f32,
            sigma_dec_fit: sigma_dec_fit as f32,
            pm_ra: pm_ra as f32,
            pm_dec: pm_dec as f32,
            sigma_pm_ra: sigma_pm_ra as f32,
            sigma_pm_dec: sigma_pm_dec as f32,
            pm_prob: pm_prob as f32,
            epoch: epoch as f32,
            n_detections,
            diffraction_spike,
            motion_catalog,
            ys4,
            observations,
        })
    }
}

pub struct USNOBFile {
    file: File,
}

impl Deref for USNOBFile {
    type Target = File;
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

pub struct USNOBFileIter<'a> {
    index: usize,
    reader: BufReader<&'a File>,
}

impl<'a> Iterator for USNOBFileIter<'a> {
    type Item = USNOBObject;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; USNOB_RECORD_SIZE];

        self.reader.read_exact(&mut buffer).ok()?;

        self.index += 1;

        USNOBObject::from_bytes(&buffer, self.index).ok()
    }
}

impl USNOBFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;

        Ok(USNOBFile { file })
    }

    pub fn iter(&self) -> USNOBFileIter {
        USNOBFileIter {
            index: 0,
            reader: BufReader::new(&self.file),
        }
    }
}
