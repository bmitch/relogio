use chrono::{Date, TimeZone, Utc};
use serde::{de, Deserialize};
use serde_json::{Value, from_str, json};
use std::fmt;
use std::error::Error;

fn get_moon_phase(dt: Date<Utc>) -> MoonPhase {
    let moon_json = get_usno_json(dt).expect("Problem getting data from USNO");
    process_moon_data(&moon_json).expect("Problem trying to parse json")
}

fn get_usno_json(dt: Date<Utc>) -> reqwest::Result<String> {
    let url = dt
        .format("https://api.usno.navy.mil/moon/phase?date=%m/%d/%Y&nump=1")
        .to_string();

    reqwest::get(&url)?.text()
}

#[derive(Debug)]
struct PhaseError {
    found: Option<String>,
}

impl PhaseError {
    fn new(s: Option<&str>) -> PhaseError {
        PhaseError {found: s.map(|s| s.to_string())}
    }
}

impl fmt::Display for PhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\
Expected one of `New Moon`, `First Quarter`, \
`Full Moon`, or `Last Quarter`, got {:?}",
                self.found
                )
    }
}

impl Error for PhaseError { }

fn process_moon_data(moon_json: &str) -> Result<MoonPhase, Box<dyn Error>> {
    let data: Value = from_str(moon_json)?;
    match data["phasedata"][0]["phase"].as_str() {
        Some("New Moon") => Ok(MoonPhase::New),
        Some("First Quarter") => Ok(MoonPhase::FirstQuarter),
        Some("Full Moon") => Ok(MoonPhase::Full),
        Some("Last Quarter") => Ok(MoonPhase::LastQuarter),
        s => Err(Box::new(PhaseError::new(s))),
    }
}

#[derive(Deserialize, Debug)]
struct MoonData {
    error: bool,
    apiversion: String,
    year: i32,
    month: u32,
    day: u32,
    numphases: usize,
    datechanged: bool,
    phasedata: Vec<PhaseDate>,
}

#[derive(Deserialize, Debug)]
struct PhaseDate {
    phase: MoonPhase,
    date: String,
    time: String,
}

#[derive(Debug, PartialEq)]
enum MoonPhase {
    New,
    FirstQuarter,
    Full,
    LastQuarter,
}

impl<'de> Deserialize<'de> for MoonPhase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>,
    {
        struct MoonPhaseVisitor;

        impl<'de> de::Visitor<'de> for MoonPhaseVisitor {

            type Value = MoonPhase;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "One of `New Moon`, `First Quarter`, `Full Moon`, or `Last Quarter`"
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<MoonPhase, E>
                where E: de::Error,
            {
                match s {
                    "New Moon" => Ok(MoonPhase::New),
                    "First Quarter" => Ok(MoonPhase::FirstQuarter),
                    "Full Moon" => Ok(MoonPhase::Full),
                    "Last Quarter" => Ok(MoonPhase::LastQuarter),
                    _ => Err(de::Error::unknown_variant(s, VARIANTS)),
                }
            }
        }

        const VARIANTS: &'static [&'static str] =
            &["New Moon", "First Quarter", "Full Moon", "Last Quarter"];

        deserializer.deserialize_str(MoonPhaseVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // this is a slow test that actually hits the USNO api
    // and it will break without internet
    #[ignore]
    fn test_get_moon_data() {
        get_moon_phase(Utc.ymd(2019, 03, 17));
    }

    #[test]
    fn test_process_moon_data() {
        // march 2019 supermoon
        let moon_json = "{
      \"error\":false,
      \"apiversion\":\"2.2.1\",
      \"year\":2019,
      \"month\":3,
      \"day\":20,
      \"numphases\":1,
      \"datechanged\":false,
      \"phasedata\":[
            {
               \"phase\":\"Full Moon\",
               \"date\":\"2019 Mar 21\",
               \"time\":\"01:43\"
            }
      ]
   }";
        let phase = process_moon_data(moon_json).unwrap();
        assert_eq!(phase, MoonPhase::Full);
    }

    #[test]
    fn test_process_bad_json() {
        let moon_json = "{
      \"error\":false,
      \"apiversion\":\"2.2.1\",
      \"year\":2019,
      \"month\":3,
      \"day\":20,
      \"nump"; // cut off here
        assert!(process_moon_data(moon_json).is_err())
    }
}
