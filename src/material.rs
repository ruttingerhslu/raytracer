use crate::color::Color;
use crate::ray::Ray;
use crate::vec3;
use crate::hittable::HitRecord;
 
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn albedo(&self) -> Color {
        Color::new(1.0, 1.0, 1.0) // default for non-colored materials
    }
}

pub struct Lambertian {
    albedo: Color,
}
 
impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}
 
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }

    fn albedo(&self) -> Color {
        self.albedo
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
 
impl Metal {
    pub fn new(a: Color, f: f32) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}
 
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

    fn albedo(&self) -> Color {
        self.albedo
    }
}

pub struct Glass {
    albedo: Color,
    refractive_index: f32,
}

impl Glass {
    pub fn new(a: Color, ior: f32) -> Glass {
        Glass {
            albedo: a,
            refractive_index: ior,
        }
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let incident = vec3::unit_vector(r_in.direction());
        let mut normal = rec.normal;
        let mut eta_i = r_in.current_ior();
        let mut eta_t = self.refractive_index as f32;
        let mut cos_theta = vec3::dot(-incident, normal);

        if cos_theta < 0.0 {
            normal = -normal;
            std::mem::swap(&mut eta_i, &mut eta_t);
            cos_theta = vec3::dot(-incident, normal);
        }

        let eta = eta_i / eta_t;
        let refracted = vec3::refract(incident, normal, eta);
        let reflect_prob = if refracted.is_none() {
            1.0
        } else {
            vec3::schlick(cos_theta, eta_t)
        };

        let direction = if rand::random::<f32>() < reflect_prob {
            vec3::reflect(incident, normal)
        } else {
            refracted.unwrap()
        };

        *scattered = Ray::with_ior(rec.p, direction, if rec.front_face { self.refractive_index } else { 1.0 });
        *attenuation = Color::new(1.0, 1.0, 1.0);
        true
    }

    fn albedo(&self) -> Color {
        self.albedo
    }
}

