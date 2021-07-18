use thiserror::Error;

use enum_index_derive::{IndexEnum};
use variant_count::VariantCount;

mod random_grund;

pub fn get_grund() -> Grund {
    rand::random()
}

#[non_exhaustive]
#[derive(Debug, Error, VariantCount, IndexEnum)]
pub enum Grund {
    #[error("Fahrt faellt aus")]
    FahrtFaelltAus,

    #[error("Fährt heute nur bis {0}")]
    FaehrtHeuteNurBis(String),

    #[error("Verzögerungen im Betriebsablauf")]
    VerzoegerungenImBetriebsablauf,

    #[error("Gleiswechsel")]
    Gleiswechsel,

    #[error("Fahrt fällt aus, es verkehrt Ersatzfahrt {0}")]
    FahrtFaelltAusMitErsatzfahrt(String),

    #[error("Technische Störungen am Zug")]
    TechnischeStoerungenAmZug,

    #[error("Verspätung eines vorausfahrenden Zuges")]
    VerspaetungEinesVorausfahrendenZuges,

    #[error("Verspätete Bereitstellung des Zuges")]
    VerspaeteteBereitstellungDesZuges,

    #[error("Bauarbeiten")]
    Bauarbeiten,

    #[error("Witterungsbedingte Störung")]
    WitterungsbedingteStoerung,

    #[error("Weichenstörung")]
    WeichenStoerung,

    #[error("Änderung im Fahrtverlauf")]
    AenderungImFahrtverlauf,

    #[error("Unwetter")]
    Unwetter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bauarbeiten() {
        let formatted = format!("Grund: {}", Grund::Bauarbeiten);
        assert_eq!("Grund: Bauarbeiten", formatted);
    }

    #[test]
    fn get_random_grund() {
        println!("Grund: {}", get_grund());
    }
}
