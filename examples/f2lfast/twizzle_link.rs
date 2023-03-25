use cubing::alg::Alg;
use url::Url;

pub fn twizzle_link(scramble: &Alg, solution: &Alg) -> String {
    let mut url = Url::parse("https://alpha.twizzle.net/edit/").unwrap();
    url.query_pairs_mut()
        .append_pair("setup-alg", &scramble.to_string());
    url.query_pairs_mut()
        .append_pair("alg", &solution.to_string());
    url.to_string()
}
