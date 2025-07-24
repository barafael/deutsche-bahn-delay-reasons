use super::Grund;

use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use strum::EnumCount;

/// Implement a standard distribution for our central delay code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
/// All members of enum variants must be default-constructible for that to work.
impl Distribution<Grund> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grund {
        let index = rng.random_range(0..Grund::COUNT);

        // unwrap here is OK by construction, given that the VARIANT_COUNT is derived correctly.
        Grund::from_repr(index).expect("range is guaranteed to be in-bounds")
    }
}
