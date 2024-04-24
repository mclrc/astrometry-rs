#!/bin/bash

# Download the 4100 series FITS index files
for i in $(seq -f "%02g" 7 19); do
	wget -N -P 4100/ http://data.astrometry.net/4100/index-41$i.fits
done
