#!/usr/bin/env python3
from astropy.io import fits
import sys

def print_fits_rows(fits_file_path, hdu_index=1):
    # Open the FITS file with memory mapping enabled
    with fits.open(fits_file_path, memmap=True) as hdul:
        hdu = hdul[hdu_index]

        # Access the data in the HDU
        for row in hdu.data:
            print(row)  # Print each row

if __name__ == "__main__":
    print_fits_rows(sys.argv[1])
