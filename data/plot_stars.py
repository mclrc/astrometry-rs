import sys
import numpy as np
import matplotlib.pyplot as plt
from astropy.io import fits

def main():
    fits_file = sys.argv[1]

    with fits.open(fits_file, memmap=True) as hdul:
        data = hdul[1].data
        ra = data['RA']
        dec = data['DEC']

    ra_rad = np.radians(ra)
    dec_rad = np.radians(dec)

    x = np.cos(dec_rad) * np.cos(ra_rad)
    y = np.cos(dec_rad) * np.sin(ra_rad)
    z = np.sin(dec_rad)

    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    ax.scatter(x, y, z, c='blue', s=1)

    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')
    
    ax.set_xticklabels([])
    ax.set_yticklabels([])
    ax.set_zticklabels([])

    plt.rcParams['toolbar'] = 'None'

    plt.show()

if __name__ == "__main__":
    main()
