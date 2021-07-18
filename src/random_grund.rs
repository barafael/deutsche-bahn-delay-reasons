use super::Grund;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use enum_index::IndexEnum;

impl Distribution<Grund> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grund {
        let index = rng.gen_range(0..Grund::VARIANT_COUNT);
        Grund::index_enum(index).unwrap()
    }
}
