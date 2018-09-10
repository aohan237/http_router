extern crate regex;

pub struct UrlMatcher {
    urls: Vec<&'static str>,
    match_set: regex::RegexSet,
}
impl UrlMatcher {
    pub fn new() -> UrlMatcher {
        UrlMatcher {
            urls: Vec::new(),
            match_set: regex::RegexSet::new(&[""]).unwrap(),
        }
    }
    pub fn add_url(&mut self, url: &'static str) {
        self.urls.push(url);
        self.match_set = regex::RegexSet::new(self.urls.as_slice()).unwrap()
    }
    pub fn gen_url_regex_set(&self) -> regex::RegexSet {
        regex::RegexSet::new(self.urls.as_slice()).unwrap()
    }
    pub fn match_url(&self, url: &str) -> &'static str {
        let matches: Vec<_> = self.match_set.matches(url).into_iter().collect();
        match matches.is_empty() {
            true => "",
            false => {
                println!("{:?},{:?}", url, matches);
                let index = matches[0];
                self.urls[index]
            }
        }
    }
}
