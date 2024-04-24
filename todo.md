# TODO

## Index building
### USNO-B1
- Parse .acc/.cat files from USNO-B1
  - Convert to FITS
  - Maybe compile astrometry.net .cat parser?
- Should this be done in python?
- Alternatively: select smaller catalogue available in FITS format (SDSS?) for now
- Alternatively: use astrometry.net index files for now
- Relevant astrometry.net code
  - usnobtofits
  - startree

# DB/Index setup
- Figure out RA/Dec <-> pixel space projection for quad building
- Write (RA,Dec,Nside) -> HEALPix implementation in SQL to build DB indices
- Re-read paper for index segmentation, cell sizes, etc
  - Stars per HEALPix
  - Size of HEALPix
    - How to go from assumed image size to Nside?

## Image processing
- Investigate rust image processing libraries

## Maths
- Read up on Bayesian decision theory
- Read relevant C code
