use rand::Rng;

pub type RayTracingFloat = f64;

pub fn random_double(min: &RayTracingFloat, max: &RayTracingFloat) -> RayTracingFloat {
    thread_local!(static RNG_STORAGE : std::cell::RefCell<rand::prelude::ThreadRng> 
        = std::cell::RefCell::new(rand::thread_rng()));
    return RNG_STORAGE.with(|rng_ref| return rng_ref.borrow_mut().gen_range(*min..*max));
}

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        return min;
    } else if input > max {
        return max;
    } else {
        return input;
    }
}
