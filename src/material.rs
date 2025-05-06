use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{self, Vec3, Point3};
use crate::hittable::HitRecord;
 
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn albedo(&self) -> Color; 
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
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
        return self.albedo;
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
 
impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
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
        return self.albedo;
    }
}

pub struct Glass {
    albedo: Color,
    refractive_index: f32,
}

impl Glass {
    pub fn new(albedo: Color, ior: f32) -> Glass {
        Glass {
            albedo,
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
        let mut eta_t = self.refractive_index;
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

        *scattered = Ray::new(rec.p, direction);
        *attenuation = self.albedo;
        true
    }

    fn albedo(&self) -> Color {
        return self.albedo;
    }
}

pub struct SoapBubble {
    refractive_index: f32,
}

impl SoapBubble {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Material for SoapBubble {
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
        let mut eta_t = self.refractive_index;
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

        *scattered = Ray::with_ior(
            rec.p,
            direction,
            if rec.front_face { self.refractive_index } else { 1.0 }
        );

        *attenuation = interference_color(cos_theta, rec.p, rec.normal);
        true
    }

    fn albedo(&self) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}

fn interference_color(cos_theta: f32, position: Point3, normal: Vec3) -> Color {
    let view_dir = -vec3::unit_vector(position); // Approximate view direction
    let angle = vec3::dot(view_dir, normal).clamp(0.0, 1.0); // angle of incidence

    // Use angle to vary film thickness (more thickness at glancing angles)
    let angle_thickness_boost = 1.0 + (1.0 - angle) * 2.0;

    // Simulate spatial thickness variation (e.g., due to surface imperfections)
    let noise = 0.5 + 0.5 * (10.0 * position.x().sin() * position.y().cos());

    // Effective optical path difference
    let film_thickness = 0.3 * angle_thickness_boost * noise;

    let phase = 2.0 * std::f32::consts::PI * film_thickness;

    // Simple sine-based interference color (could use real wavelength math)
    let r = 0.5 + 0.5 * (phase).sin();
    let g = 0.5 + 0.5 * (phase * 1.3).sin();
    let b = 0.5 + 0.5 * (phase * 1.7).sin();

    Color::new(r, g, b)
}

