use super::Grund;

use rand::Rng;
use rand::RngExt;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;
use strum::EnumCount;

/// Realistic destinations for "Fährt heute nur bis {0}".
const STATIONS: &[&str] = &[
    "Hamburg Hbf",
    "München Hbf",
    "Köln Hbf",
    "Frankfurt (Main) Hbf",
    "Berlin Ostbahnhof",
    "Hannover Hbf",
    "Bielefeld Hbf",
    "Freiburg (Breisgau) Hbf",
    "Osnabrück Hbf",
    "Wolfsburg",
];

/// Train designations for "Statt {0} fährt heute {1}. Tickets behalten
/// weiterhin ihre Gültigkeit."
const TRAINS: &[&str] = &[
    "ICE 506", "ICE 2926", "ICE 884", "ICE 1085", "IC 2034", "IC 2381", "RE 4408", "RB 27142",
    "S 3",
];

/// Replacement service designations for "es verkehrt Ersatzfahrt {0}".
const REPLACEMENTS: &[&str] = &["ICE 2506", "IC 2036", "Bus SEV 1", "Bus SEV 2", "RE 10126"];

fn pick<R: Rng + ?Sized>(rng: &mut R, list: &[&str]) -> String {
    list.choose(rng)
        .copied()
        .expect("lists are non-empty")
        .to_owned()
}

/// Implement a standard distribution for our central delay code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
///
/// Variants carrying a payload fill it from a curated list of realistic
/// values. Strum's `from_repr` would construct such variants with empty
/// payloads, so they are matched out and re-filled here. The catch-all match
/// arm keeps this in sync with future payload-carrying variants at compile
/// time.
impl Distribution<Grund> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grund {
        let index = rng.random_range(0..Grund::COUNT);
        match Grund::from_repr(index).expect("range is guaranteed to be in-bounds") {
            Grund::FaehrtHeuteNurBis(_) => Grund::FaehrtHeuteNurBis(pick(rng, STATIONS)),
            Grund::FahrtFaelltAusMitErsatzfahrt(_) => {
                Grund::FahrtFaelltAusMitErsatzfahrt(pick(rng, REPLACEMENTS))
            }
            Grund::StattZugFaehrtHeuteZug(_, _) => {
                Grund::StattZugFaehrtHeuteZug(pick(rng, TRAINS), pick(rng, TRAINS))
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn payloads_are_filled_from_lists() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut seen_nur_bis = false;
        let mut seen_ersatzfahrt = false;
        let mut seen_statt_zug = false;
        for _ in 0..2000 {
            match StandardUniform.sample(&mut rng) {
                Grund::FaehrtHeuteNurBis(station) => {
                    seen_nur_bis = true;
                    assert!(STATIONS.contains(&station.as_str()));
                }
                Grund::FahrtFaelltAusMitErsatzfahrt(replacement) => {
                    seen_ersatzfahrt = true;
                    assert!(REPLACEMENTS.contains(&replacement.as_str()));
                }
                Grund::StattZugFaehrtHeuteZug(from, to) => {
                    seen_statt_zug = true;
                    assert!(TRAINS.contains(&from.as_str()));
                    assert!(TRAINS.contains(&to.as_str()));
                }
                _ => {}
            }
        }
        assert!(seen_nur_bis && seen_ersatzfahrt && seen_statt_zug);
    }
}
