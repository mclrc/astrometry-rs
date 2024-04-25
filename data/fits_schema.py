#!/usr/bin/env python3
from astropy.io import fits
import sys

def print_fits_schema(file_path):
    with fits.open(file_path) as hdul:
        hdul.info()
        print(hdul[1])
        data = hdul[1].data
        print(data.columns)
        for column in data.columns:
            print(column.name, column.format)

if __name__ == "__main__":
    print_fits_schema(sys.argv[1])
