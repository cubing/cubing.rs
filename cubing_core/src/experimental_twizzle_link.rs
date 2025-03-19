use url::Url;

use crate::alg::Alg;

pub enum ExperimentalSetupAnchor {
    Start,
    End,
}

impl Default for ExperimentalSetupAnchor {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Default)]
pub struct ExperimentalTwizzleLinkParameters<'a> {
    pub setup: Option<&'a Alg>,
    pub alg: Option<&'a Alg>,
    pub puzzle: Option<&'a str>,
    pub stickering: Option<&'a str>,
    pub setup_anchor: Option<&'a str>,
}

pub fn experimental_twizzle_link(parameters: ExperimentalTwizzleLinkParameters) -> String {
    let mut url = Url::parse("https://alpha.twizzle.net/edit/").unwrap();
    if let Some(setup) = parameters.setup {
        url.query_pairs_mut()
            .append_pair("setup-alg", &setup.to_string());
    }
    if let Some(alg) = parameters.alg {
        url.query_pairs_mut().append_pair("alg", &alg.to_string());
    }
    if let Some(puzzle) = parameters.puzzle {
        url.query_pairs_mut().append_pair("puzzle", puzzle);
    }
    if let Some(setup_anchor) = parameters.setup_anchor {
        url.query_pairs_mut()
            .append_pair("setup_anchor", setup_anchor);
    }
    if let Some(stickering) = parameters.stickering {
        url.query_pairs_mut().append_pair("stickering", stickering);
    }
    url.to_string()
}
