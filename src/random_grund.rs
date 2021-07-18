use super::Grund;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use enum_index::IndexEnum;

/// Implement a standard distribution for our central delay code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
/// All members of enum variants must be default-constructible for that to work.
impl Distribution<Grund> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grund {
        let index = rng.gen_range(0..Grund::VARIANT_COUNT);

        // unwrap here is OK by construction, given that the VARIANT_COUNT is derived correctly.
        Grund::index_enum(index).unwrap()
    }
}
