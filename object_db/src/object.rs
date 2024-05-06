use sqlx::{query, Error, QueryBuilder, Sqlite, SqliteConnection};

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

impl Object {
    #[allow(dead_code)]
    async fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Error> {
        query!(
            "INSERT INTO object (usnob_id, ra, sigma_ra, sigma_ra_fit, pm_ra, dec, sigma_dec, sigma_dec_fit, pm_dec, bmag, rmag, imag, epoch, num_detections, origin_file) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
            self.usnob_id,
            self.ra,
            self.sigma_ra,
            self.sigma_ra_fit,
            self.pm_ra,
            self.dec,
            self.sigma_dec,
            self.sigma_dec_fit,
            self.pm_dec,
            self.bmag,
            self.rmag,
            self.imag,
            self.epoch,
            self.num_detections,
            self.origin_file
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn insert_many(
        objects: impl Iterator<Item = Object>,
        conn: &mut SqliteConnection,
    ) -> Result<(), Error> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("INSERT OR REPLACE INTO object (usnob_id, ra, sigma_ra, sigma_ra_fit, pm_ra, dec, sigma_dec, sigma_dec_fit, pm_dec, bmag, rmag, imag, epoch, num_detections, origin_file) ");

        query_builder.push_values(objects, |mut b, object| {
            b.push_bind(object.usnob_id.clone())
                .push_bind(object.ra)
                .push_bind(object.sigma_ra)
                .push_bind(object.sigma_ra_fit)
                .push_bind(object.pm_ra)
                .push_bind(object.dec)
                .push_bind(object.sigma_dec)
                .push_bind(object.sigma_dec_fit)
                .push_bind(object.pm_dec)
                .push_bind(object.bmag)
                .push_bind(object.rmag)
                .push_bind(object.imag)
                .push_bind(object.epoch)
                .push_bind(object.num_detections)
                .push_bind(object.origin_file.clone());
        });

        let query = query_builder.build();

        query.execute(conn).await?;

        Ok(())
    }
}
