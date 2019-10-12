use chrono::{Date, TimeZone, Utc};
use reqwest_mock::{Client, DirectClient};
use serde_json::{from_str, Value};
use std::error::Error;
use std::fmt;

fn get_moon_phase(dt: Date<Utc>) -> MoonPhase {
    let moon_json = new_client()
        .get_usno_json(dt)
        .expect("Problem getting data from USNO");
    process_moon_data(&moon_json).expect("Problem trying to parse json")
}

// An abstraction of a reqwest client that may hold a real client or a stub
struct MyClient<C: Client> {
    client: C,
}

impl<C: Client> MyClient<C> {
    fn get_usno_json(&self, dt: Date<Utc>) -> Result<String, reqwest_mock::error::Error> {
        let url = dt
            .format("https://api.usno.navy.mil/moon/phase?date=%m/%d/%Y&nump=1")
            .to_string();

        self.client.get(&url).send()?.body_to_utf8()
    }
}

// Use a DirectClient for the actual application
fn new_client() -> MyClient<DirectClient> {
    MyClient { client: DirectClient::new() }
}

#[derive(Debug)]
struct PhaseError {
    found: Option<String>,
}

impl PhaseError {
    fn new(s: Option<&str>) -> PhaseError {
        PhaseError { found: s.map(|s| s.to_string()) }
    }
}

impl fmt::Display for PhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
Expected one of `New Moon`, `First Quarter`, \
`Full Moon`, or `Last Quarter`, got {:?}",
            self.found
        )
    }
}

impl Error for PhaseError {}

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

#[derive(Debug, PartialEq)]
enum MoonPhase {
    New,
    FirstQuarter,
    Full,
    LastQuarter,
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest_mock::{Method, StubClient, StubDefault, StubSettings, StubStrictness, Url};

    // A stub client for testing using the March 2019 Supermoon
    fn test_client() -> MyClient<StubClient> {
        let mut client = StubClient::new(StubSettings {
            default: StubDefault::Error,
            strictness: StubStrictness::MethodUrl,
        });
        assert!(client
            .stub(
                Url::parse("https://api.usno.navy.mil/moon/phase?date=03/17/2019&nump=1").unwrap()
            )
            .method(Method::GET)
            .response()
            .body(
                "{
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
   }"
            )
            .mock()
            .is_ok());

        MyClient { client: client }
    }

    #[test]
    // this is a slow test that actually hits the USNO api
    // and it will break without internet
    #[ignore]
    fn test_get_moon_data() {
        get_moon_phase(Utc.ymd(2019, 03, 17));
    }

    #[test]
    fn test_get_json_data_for_given_date() {
        assert!(test_client().get_usno_json(Utc.ymd(2019, 03, 17)).is_ok());
    }

    #[test]
    fn test_process_moon_data() {
        // march 2019 supermoon
        let moon_json = test_client().get_usno_json(Utc.ymd(2019, 03, 17)).unwrap();
        let phase = process_moon_data(&moon_json).unwrap();
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

    #[test]
    fn test_invalid_moon_phase() {
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
               \"phase\":\"Blue Moon\",
               \"date\":\"2019 Mar 21\",
               \"time\":\"01:43\"
            }
      ]
   }"; // blue moon is not a phase
        assert!(process_moon_data(moon_json).is_err())
    }

}
