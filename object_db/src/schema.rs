// @generated automatically by Diesel CLI.

diesel::table! {
    object (usnob_id) {
        #[max_length = 12]
        usnob_id -> Bpchar,
        ra -> Nullable<Float8>,
        sigma_ra -> Nullable<Float4>,
        sigma_ra_fit -> Nullable<Float4>,
        pm_ra -> Nullable<Float4>,
        dec -> Nullable<Float8>,
        sigma_dec -> Nullable<Float4>,
        sigma_dec_fit -> Nullable<Float4>,
        pm_dec -> Nullable<Float4>,
        bmag -> Nullable<Float4>,
        rmag -> Nullable<Float4>,
        imag -> Nullable<Float4>,
        epoch -> Nullable<Float4>,
        num_detections -> Nullable<Int4>,
        #[max_length = 8]
        flags -> Nullable<Bit>,
        origin_file -> Nullable<Text>,
    }
}
