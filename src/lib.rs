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
///
/// Each variant's source is annotated in its doc comment. Variants citing an
/// "IRIS-TTS code" reference DB's IRIS-TTS message catalogue, as documented in
/// the community-maintained listing at
/// [derf/Travel-Status-DE-IRIS](https://github.com/derf/Travel-Status-DE-IRIS/blob/master/lib/Travel/Status/DE/IRIS/Result.pm).
#[non_exhaustive]
#[derive(Debug, FromReprMacro, EnumCountMacro, Error)]
pub enum Grund {
    /// Trip cancelled.
    ///
    /// Source: board text, rendered from IRIS cancellations by
    /// [db-fakedisplay](https://github.com/derf/db-fakedisplay/blob/master/lib/DBInfoscreen/Controller/Stationboard.pm)
    /// and captured verbatim in DB HAFAS responses
    /// ([kpublictransport fixtures](https://invent.kde.org/libraries/kpublictransport/-/blob/master/autotests/data/db-hafas/journey-nowalk.in.json)).
    #[error("Fahrt fällt aus")]
    FahrtFaelltAus,

    /// Stop cancelled.
    ///
    /// Source: DB realtime note "Halt entfällt", handled by
    /// [db-vendo-client](https://github.com/public-transport/db-vendo-client/blob/main/parse/remarks.js).
    #[error("Halt entfällt")]
    HaltEntfaellt,

    /// Trip only reaches a certain stop today.
    ///
    /// Source: board text, generated verbatim by the LEIBit station display
    /// software
    /// ([DepartureBoardViewModel.cs](https://github.com/jannikbecker/leibit/blob/master/Leibit.Client.WPF/Windows/Display/ViewModels/DepartureBoardViewModel.cs)).
    #[error("Fährt heute nur bis {0}")]
    FaehrtHeuteNurBis(String),

    /// Delays in operations.
    ///
    /// Source: IRIS-TTS code 99, "Verzögerungen im Betriebsablauf".
    #[error("Verzögerungen im Betriebsablauf")]
    VerzoegerungenImBetriebsablauf,

    /// Change of track.
    ///
    /// Source: real-time message "Gleiswechsel" captured in
    /// [hafas-client fixtures](https://github.com/public-transport/hafas-client/blob/main/test/fixtures/db-netz-remarks.json);
    /// see also
    /// [Gleiswechselbetrieb](https://de.wikipedia.org/wiki/Gleiswechselbetrieb).
    #[error("Gleiswechsel")]
    Gleiswechsel,

    /// Delay from earlier journey.
    ///
    /// Source: IRIS-TTS code 48, "Verspätung aus vorheriger Fahrt".
    #[error("Verspätung aus vorheriger Fahrt")]
    VerspaetungAusVorherigerFahrt,

    /// Person on the tracks.
    ///
    /// Source: IRIS-TTS code 7, "Unbefugte Personen auf der Strecke".
    #[error("Person auf dem Gleis")]
    PersonAufDemGleis,

    /// Signal malfunction.
    ///
    /// Source: IRIS-TTS code 34, "Defekt an einem Signal".
    #[error("Signalstörung")]
    Signalstörung,

    /// Disturbance by vandalism.
    ///
    /// Source: IRIS-TTS code 15, "Beeinträchtigung durch Vandalismus".
    #[error("Beeinträchtigung durch Vandalismus")]
    BeeintraechtigungDurchVandalismus,

    /// Door malfunction.
    ///
    /// Source: IRIS-TTS code 61, "Defekte Tür".
    #[error("Türstörung")]
    TuerStoerung,

    /// Waiting for passengers using same connection.
    ///
    /// Source: IRIS-TTS code 21, "Warten auf Anschlussreisende".
    #[error("Warten auf Anschlussreisende")]
    WartenAufAnschlussreisende,

    /// Medical emergency on the track.
    ///
    /// Source: IRIS-TTS code 8, "Notarzteinsatz auf der Strecke".
    #[error("Notarzteinsatz an der Strecke")]
    NotarztEinsatzAnDerStrecke,

    /// Medical emergency on the train.
    ///
    /// Source: IRIS-TTS code 5, "Ärztliche Versorgung eines Fahrgastes".
    #[error("Ärztliche Versorgung eines Fahrgastes")]
    AerztlicheVersorgungEinesFahrgastes,

    /// Train advancing with diminished velocity.
    ///
    /// Source: IRIS-TTS codes 42 ("Außerplanmäßige
    /// Geschwindigkeitsbeschränkung") and 69 ("Zug verkehrt mit verminderter
    /// Geschwindigkeit").
    #[error("Zug verkehrt mit verminderter Geschwindigkeit")]
    ZugVerkehrtMitVerminderterGeschwindigkeit,

    /// Trip cancelled, there is replacement trip.
    ///
    /// Source: board text; see
    /// [Ersatzverkehr](https://de.wikipedia.org/wiki/Ersatzverkehr).
    #[error("Fahrt fällt aus, es verkehrt Ersatzfahrt {0}")]
    FahrtFaelltAusMitErsatzfahrt(String),

    /// Technical difficulties with the train.
    ///
    /// Source: IRIS-TTS code 36, "Technische Störung am Zug".
    #[error("Technische Störungen am Zug")]
    TechnischeStoerungenAmZug,

    /// Delay of a previous train.
    ///
    /// Source: IRIS-TTS code 43, "Verspätung eines vorausfahrenden Zuges".
    #[error("Verspätung eines vorausfahrenden Zuges")]
    VerspaetungEinesVorausfahrendenZuges,

    /// Delayed allocation of the train.
    ///
    /// Source: IRIS-TTS code 47, "Verspätete Bereitstellung".
    #[error("Verspätete Bereitstellung des Zuges")]
    VerspaeteteBereitstellungDesZuges,

    /// Construction work.
    ///
    /// Source: IRIS-TTS code 31, "Bauarbeiten".
    #[error("Bauarbeiten")]
    Bauarbeiten,

    /// Weather-related difficulties.
    ///
    /// Source: IRIS-TTS code 22, "Witterungsbedingte Beeinträchtigungen".
    #[error("Witterungsbedingte Störung")]
    WitterungsbedingteStoerung,

    /// Switch malfunction.
    ///
    /// Source: IRIS-TTS code 64, "Defekt an einer Weiche".
    #[error("Weichenstörung")]
    Weichenstoerung,

    /// Changes in the journey course.
    ///
    /// Source: display text on ICE passenger information screens; no stable
    /// public source located. Related IRIS-TTS codes: 58 ("Umleitung"),
    /// 54 ("Verfügbarkeit der Gleise derzeit eingeschränkt").
    #[error("Änderung im Fahrtverlauf")]
    AenderungImFahrtverlauf,

    /// Storm or bad weather.
    ///
    /// Source: IRIS-TTS codes 11 ("Unwetter") and 53 ("Unwetterauswirkungen").
    #[error("Unwetter")]
    Unwetter,

    /// Short-term staff shortage.
    ///
    /// Source: IRIS-TTS codes 4, 49 ("Kurzfristiger Personalausfall") and 50
    /// ("Kurzfristige Erkrankung von Personal").
    #[error("Kurzfristiger Personalausfall")]
    KurzfristigerPersonalausfall,

    /// Different trip instead of the planned one. Tickets stay valid.
    ///
    /// Source: captured verbatim from a DB HAFAS response in
    /// [kpublictransport test data](https://invent.kde.org/libraries/kpublictransport/-/blob/master/autotests/data/db-hafas/journey-nowalk.in.json):
    /// "Statt ICE 506 fährt heute ICE 2926. Tickets behalten weiterhin ihre
    /// Gültigkeit."
    #[error("Statt {0} fährt heute {1}. Tickets behalten weiterhin ihre Gültigkeit.")]
    StattZugFaehrtHeuteZug(String, String),

    /// Train passed a stop signal without authorization (dangerous event).
    ///
    /// Source: "Vorbeifahrt am Haltbegriff" is DB terminology for a signal
    /// passed at danger; see
    /// [Signal passed at danger](https://en.wikipedia.org/wiki/Signal_passed_at_danger).
    #[error("Gefährliches Ereignis - Vorbeifahrt am Haltbegriff")]
    DangerousEventStopSignalPassedWithoutAuthorization,

    #[cfg(feature = "inofficial")]
    /// Reason being that the train must urgently visit a workshop.
    ///
    /// Source: inofficial joke variant, no public source.
    #[error("Grund ist, dass der Zug dringend in die Werkstatt muss")]
    GrundIstDassDerZugDringendInDieWerkstattMuss,

    /// Rocks on the track, need to check for damage
    ///
    /// Source: first heard on 22.10.2025,
    /// [PR #4](https://github.com/barafael/deutsche-bahn-delay-reasons/pull/4).
    #[error("Wegen Steinen auf der Strecke muss der Zug auf Schäden kontrolliert werden.")]
    SteineAufStrecke,

    /// Overhead line malfunction.
    ///
    /// Source: IRIS-TTS code 33, "Defekt an der Oberleitung".
    #[error("Oberleitungsstörung")]
    Oberleitungsstoerung,

    /// Signal box (interlocking) malfunction.
    ///
    /// Source: IRIS-TTS codes 40 ("Defektes Stellwerk") and 30
    /// ("Personalausfall im Stellwerk").
    #[error("Stellwerksstörung")]
    Stellwerksstoerung,

    /// Route closure.
    ///
    /// Source: IRIS-TTS code 35, "Streckensperrung".
    #[error("Streckensperrung")]
    Streckensperrung,

    /// Diversion.
    ///
    /// Source: IRIS-TTS code 58, "Umleitung".
    #[error("Umleitung")]
    Umleitung,

    /// Level crossing malfunction.
    ///
    /// Source: IRIS-TTS codes 41 ("Defekt an einem Bahnübergang") and 19
    /// ("Unfall an einem Bahnübergang").
    #[error("Bahnübergangsstörung")]
    Bahnuebergangsstoerung,

    /// Animal on the tracks.
    ///
    /// Source: IRIS-TTS codes 20 ("Tiere im Gleis") and 10 ("Tiere auf der
    /// Strecke").
    #[error("Tier im Gleis")]
    TierImGleis,

    /// Vehicle on the tracks.
    ///
    /// Source: e.g. car on the S-Bahn tracks at Berlin Bösebrücke, 19.09.2026
    /// ([Welt](https://www.welt.de/regionales/berlin/article6aaf9f52b1857e86757a7fe5/auto-stuerzt-auf-s-bahn-gleise-strecke-wieder-frei.html));
    /// related IRIS-TTS codes: 28 ("Gegenstände auf der Strecke"), 19 ("Unfall
    /// an einem Bahnübergang").
    #[error("Fahrzeug im Gleis")]
    FahrzeugImGleis,

    /// Police operation.
    ///
    /// Source: IRIS-TTS code 2, "Polizeieinsatz".
    #[error("Polizeieinsatz")]
    Polizeieinsatz,

    /// Fire brigade deployment.
    ///
    /// Source: IRIS-TTS code 3, "Feuerwehreinsatz auf der Strecke".
    #[error("Feuerwehreinsatz")]
    Feuerwehreinsatz,

    /// Strike.
    ///
    /// Source: IRIS-TTS codes 52 ("Streik") and 9 ("Streikauswirkungen").
    #[error("Streik")]
    Streik,

    /// Border control.
    ///
    /// Source: IRIS-TTS code 13, "Pass- und Zollkontrolle".
    #[error("Grenzkontrolle")]
    Grenzkontrolle,

    /// Unusually high passenger volume.
    ///
    /// Source: IRIS-TTS code 68, "Hohes Fahrgastaufkommen".
    #[error("Hohes Fahrgastaufkommen")]
    HohesFahrgastaufkommen,

    /// Delayed boarding and alighting.
    ///
    /// Source: IRIS-TTS codes 32 ("Längere Haltezeit am Bahnhof") and 68
    /// ("Hohes Fahrgastaufkommen verlängert Ein- und Ausstieg", official
    /// wording per the catalogue's xlsx annotation).
    #[error("Verzögerungen beim Ein- und Ausstieg")]
    VerzoegerungenBeimEinUndAusstieg,

    /// Waiting for the opposing train.
    ///
    /// Source: IRIS-TTS code 44, "Warten auf einen entgegenkommenden Zug".
    #[error("Warten auf einen Gegenzug")]
    WartenAufEinenGegenzug,

    /// Waiting for a clear route.
    ///
    /// Source: board text; see
    /// [Fahrstraße](https://de.wikipedia.org/wiki/Fahrstra%C3%9Fe). Related
    /// IRIS-TTS codes: 45 ("Vorfahrt eines anderen Zuges"), 54
    /// ("Verfügbarkeit der Gleise derzeit eingeschränkt").
    #[error("Warten auf freie Fahrstraße")]
    WartenAufFreieFahrstrasse,

    /// Delay abroad, e.g. of a connecting service.
    ///
    /// Source: IRIS-TTS code 24, "Verspätung im Ausland".
    #[error("Verspätung im Ausland")]
    VerspaetungImAusland,

    /// Fluid leak from the train.
    ///
    /// Source: display text on station passenger information displays; no
    /// stable public source located. Related IRIS-TTS code: 36 ("Technische
    /// Störung am Zug").
    #[error("Austritt von Flüssigkeiten")]
    AustrittVonFluessigkeiten,

    /// Air conditioning failure.
    ///
    /// Source: e.g. "Hitzekollaps im ICE wegen defekter Klimaanlage"
    /// ([WDR](https://www1.wdr.de/archiv/jahresrueckblick/julizehnicehitze100.amp)).
    #[error("Klimaanlagenausfall")]
    Klimaanlagenausfall,
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
