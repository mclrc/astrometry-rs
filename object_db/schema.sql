CREATE TABLE IF NOT EXISTS object (
    usnob_id TEXT PRIMARY KEY,
    ra REAL,
    sigma_ra REAL,
    sigma_ra_fit REAL,
    pm_ra REAL,
    dec REAL,
    sigma_dec REAL,
    sigma_dec_fit REAL,
    pm_dec REAL,
    rmag REAL,
    bmag REAL,
    imag REAL,
    epoch REAL,
    num_detections INTEGER,
    origin_file TEXT
);
