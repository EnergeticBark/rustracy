use crate::{Color, HitRecord, random_f64, Ray, unit_vector, Vec3, vec3};

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
    pub fn new(albedo: Color) -> Self {
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
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflect_direction = unit_vector(r_in.direction()).reflect(rec.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();

        if vec3::dot(reflect_direction, rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered: Ray::new(rec.p, reflect_direction),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction
        }
    }

    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0-ref_idx)/(1.0+ref_idx)).powi(2);
        r0 + (1.0 - r0)*(1.0-cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio= if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some(ScatterResult {
            scattered: Ray::new(rec.p, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}