//! Deutsche Bahn is awesome, but often trains are late.
//! There can be many reasons, some of which this library enumerates.
//! Use the function [`Grund::default`] or [`get_grund`] to get a delay reason.
//!
//! This library exists because I was waiting for a train.

use strum_macros::{EnumCount as EnumCountMacro, FromRepr as FromReprMacro};
use thiserror::Error;

mod random_grund;

/// Get a random delay reason.
pub fn get_grund() -> Grund {
    Grund::default()
}

impl Default for Grund {
    fn default() -> Self {
        rand::random()
    }
}

/// Some of the possible reasons a Deutsche Bahn train could be delayed.
#[non_exhaustive]
#[derive(Debug, FromReprMacro, EnumCountMacro, Error)]
pub enum Grund {
    /// Trip cancelled.
    #[error("Fahrt fällt aus")]
    FahrtFaelltAus,

    /// Stop cancelled.
    #[error("Halt entfällt")]
    HaltEntfaellt,

    /// Trip only reaches a certain stop today.
    #[error("Fährt heute nur bis {0}")]
    FaehrtHeuteNurBis(String),

    /// Delays in operations.
    #[error("Verzögerungen im Betriebsablauf")]
    VerzoegerungenImBetriebsablauf,

    /// Change of track.
    #[error("Gleiswechsel")]
    Gleiswechsel,

    /// Delay from earlier journey.
    #[error("Verspätung aus vorheriger Fahrt")]
    VerspaetungAusVorherigerFahrt,

    /// Person on the tracks.
    #[error("Person auf dem Gleis")]
    PersonAufDemGleis,

    /// Signal malfunction.
    #[error("Signalstörung")]
    Signalstörung,

    /// Disturbance by vandalism.
    #[error("Beeinträchtigung durch Vandalismus")]
    BeeintraechtigungDurchVandalismus,

    /// Door malfunction.
    #[error("Tuerstoerung")]
    TuerStoerung,

    /// Waiting for passengers using same connection.
    #[error("Warten auf Anschlussreisende")]
    WartenAufAnschlussreisende,

    /// Medical emergency on the track.
    #[error("NotarztEinsatzAnDerStrecke")]
    NotarztEinsatzAnDerStrecke,

    /// Medical emergency on the train.
    #[error("Ärztliche Versorgung eines Fahrgastes")]
    AerztlicheVersorgungEinesFahrgastes,

    /// Train advancing with diminished velocity.
    #[error("Zug verkehrt mit verminderter Geschwindigkeit")]
    ZugVerkehrtMitVerminderterGeschwindigkeit,

    /// Trip cancelled, there is replacement trip.
    #[error("Fahrt fällt aus, es verkehrt Ersatzfahrt {0}")]
    FahrtFaelltAusMitErsatzfahrt(String),

    /// Technical difficulties with the train.
    #[error("Technische Störungen am Zug")]
    TechnischeStoerungenAmZug,

    /// Delay of a previous train.
    #[error("Verspätung eines vorausfahrenden Zuges")]
    VerspaetungEinesVorausfahrendenZuges,

    /// Delayed allocation of the train.
    #[error("Verspätete Bereitstellung des Zuges")]
    VerspaeteteBereitstellungDesZuges,

    /// Construction work.
    #[error("Bauarbeiten")]
    Bauarbeiten,

    /// Weather-related difficulties.
    #[error("Witterungsbedingte Störung")]
    WitterungsbedingteStoerung,

    /// Switch malfunction.
    #[error("Weichenstörung")]
    Weichenstoerung,

    /// Changes in the journey course.
    #[error("Änderung im Fahrtverlauf")]
    AenderungImFahrtverlauf,

    /// Storm or bad weather.
    #[error("Unwetter")]
    Unwetter,

    /// Short-term staff shortage.
    #[error("Kurzfristiger Personalausfall")]
    KurzfristigerPersonalausfall,

    /// Different trip instead of the planned one. Tickets stay valid.
    #[error("Statt {0} fährt heute {1}. Tickets behalten weiterhin ihre Gültigkeit.")]
    StattZugFaehrtHeuteZug(String, String),

    ///
    #[error("Gefährliches Ereignis - Vorbeifahrt am Haltbegriff")]
    DangerousEventStopSignalPassedWithoutAuthorization,

    #[cfg(feature = "inofficial")]
    /// Reason being that the train must urgently visit a workshop.
    #[error("Grund ist, dass der Zug dringend in die Werkstatt muss")]
    GrundIstDassDerZugDringendInDieWerkstattMuss,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bauarbeiten() {
        let formatted = format!("Grund: {}", Grund::Bauarbeiten);
        assert_eq!("Grund: Bauarbeiten", formatted);
    }
}
