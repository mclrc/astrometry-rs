use fitrs::{Fits, Hdu, HeaderValue};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    file: String,
    /* #[clap(long)]
    order: u32,
    #[clap(long = "sphpx")]
    stars_per_healpix: u32,
    #[clap(short, long)]
    output: String, */
}

fn find_n_fields(hdu: &Hdu, key: &str, n: i32) -> Vec<String> {
    (0..n)
        .map(|i| hdu.value(&format!("{}{}", key, i + 1)).unwrap())
        .map(|v| match v {
            HeaderValue::CharacterString(s) => s.clone(),
            v => panic!("{} Not a string: {:?}", key, v),
        })
        .collect::<Vec<_>>()
}

fn main() {
    let args = Args::parse();

    let fits = Fits::open(args.file).unwrap();

    let hdu = fits.get(1).unwrap();

    let nfields = match hdu.value("TFIELDS").unwrap() {
        HeaderValue::IntegerNumber(n) => *n,
        v => panic!("TFIELDS Not an integer: {:?}", v),
    };

    let names = find_n_fields(&hdu, "TTYPE", nfields);
    let formats = find_n_fields(&hdu, "TFORM", nfields);
    let units = find_n_fields(&hdu, "TUNIT", nfields);

    for ((name, format), unit) in names.iter().zip(formats.iter()).zip(units.iter()) {
        println!("{} {} {}", name, format, unit);
    }

    println!("{:?}", fits.get(1).unwrap().header());
    for hdu in fits.iter() {
        println!("{:?}", hdu.header().iter().map(|h| &h.0));
    }

    println!("{:?}\n\n", hdu.value("BITPIX").unwrap());
    println!("{:?}", hdu.value("NAXIS").unwrap());
    println!("{:?}", hdu.value("NAXIS1").unwrap());
    println!("{:?}", hdu.value("NAXIS2").unwrap());
    println!("{:?}", hdu.naxis());

    let data = hdu.read_data();

    println!("{:?}", data);
}
