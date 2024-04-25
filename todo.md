# TODO

## Index building
- Investigate astrometry.net FITS index format
- Answer questions about FITS file segmentation

## DB/Index setup
- Figure out RA/Dec <-> pixel space projection for quad building
- Write (RA,Dec,Nside) -> HEALPix implementation in SQL to build DB indices
- Ingest USNO-B1 into Postgres
  - 1 billion rows?
  - Which columns? Adding any after the fact would be painful
- Re-read paper for index segmentation, cell sizes, etc
  - Stars per HEALPix
  - Size of HEALPix
  - How to go from assumed image size to Nside?

## Source extraction
- Investigate rust image processing libraries

## Maths
- Read up on Bayesian decision theory
- Read relevant C code (solver.c)
