use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};

#[derive(Queryable, Selectable, Insertable, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::object)]
pub struct Object {
    pub usnob_id: String,
    pub ra: f64,
    pub sigma_ra: f32,
    pub sigma_ra_fit: f32,
    pub pm_ra: f32,
    pub dec: f64,
    pub sigma_dec: f32,
    pub sigma_dec_fit: f32,
    pub pm_dec: f32,
    pub bmag: Option<f32>,
    pub rmag: Option<f32>,
    pub imag: Option<f32>,
    pub epoch: f32,
    pub num_detections: i32,
    pub origin_file: String,
}
