#!/usr/bin/env python3
from astropy.io import fits
import sys

def print_fits_schema(file_path):
    with fits.open(file_path) as hdul:
        hdul.info()
        for hdu in hdul:
            print(hdu.header)

            if hdu.data is None:
                continue

            data = hdu.data

            print(data.columns)

            for column in data.columns:
                print(column.name, column.format, column.unit)

if __name__ == "__main__":
    print_fits_schema(sys.argv[1])
