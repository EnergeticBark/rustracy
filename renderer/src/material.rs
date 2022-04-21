use crate::{Color, HitRecord, Ray, unit_vector, Vec3, vec3::dot};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn from(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    #[allow(unused)]
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterResult {
            scattered: Ray::from(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn from(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflect_direction = unit_vector(r_in.direction()).reflect(rec.normal);

        if dot(reflect_direction, rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered: Ray::from(rec.p, reflect_direction),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}