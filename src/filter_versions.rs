use semver::Version;
use semver::VersionReq;
use semver::ReqParseError;

fn fix_exact_version_for_range(range: Option<String>) -> Result<VersionReq, ReqParseError> {
    range.map(|r| {
              Version::parse(r.as_str())
              .map(|v| VersionReq::exact(&v))
              .or_else(|_| VersionReq::parse(r.as_str()))
    }).unwrap_or(Ok(VersionReq::any()))
}

pub fn filter_and_sort(versions_strs: Vec<String>, range: Option<String>) -> Result<Vec<Version>, ReqParseError> {
    fix_exact_version_for_range(range).map(|r| {
        let mut versions: Vec<Version> = versions_strs.iter()
            .map(|s| Version::parse(s.as_str()))
            .filter_map(Result::ok)
            .filter(|v| r.matches(&v))
            .collect();

        versions.sort_unstable();
        versions
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let input = vec!["1.0.0".to_string(), "1.0.1".to_string(), "1.0.2".to_string()];
        let output = vec![Version::parse("1.0.0").unwrap(), Version::parse("1.0.1").unwrap(), Version::parse("1.0.2").unwrap()];

        assert_eq!(filter_and_sort(input, None).unwrap(), output);
    }

    #[test]
    fn output_is_sorted() {
        let input = vec!["2.0.0".to_string(), "1.0.1".to_string(), "1.0.20".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap(), Version::parse("1.0.20").unwrap(), Version::parse("2.0.0").unwrap()];

        assert_eq!(filter_and_sort(input, None).unwrap(), output);
    }

    #[test]
    fn wrong_versions_ignored() {
        let input = vec!["2.0.a".to_string(), "1.0.1".to_string(), "1...aiue".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap()];

        assert_eq!(filter_and_sort(input, None).unwrap(), output);
    }

    #[test]
    fn filter_exact() {
        let input = vec!["2.0.0".to_string(), "1.0.1".to_string(), "1.0.20".to_string(), "1.1.0".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap()];

        assert_eq!(filter_and_sort(input, Some("1.0.1".to_string())).unwrap(), output);
    }

    #[test]
    fn filter_explicit_exact() {
        let input = vec!["2.0.0".to_string(), "1.0.1".to_string(), "1.0.20".to_string(), "1.1.0".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap()];

        assert_eq!(filter_and_sort(input, Some("=1.0.1".to_string())).unwrap(), output);
    }

    #[test]
    #[should_panic]
    fn filter_bad_range_error() {
        let input = vec!["2.0.0".to_string(), "1.0.1".to_string(), "1.0.20".to_string(), "1.1.0".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap()];

        assert_eq!(filter_and_sort(input, Some("==1.0.1".to_string())).unwrap(), output);
    }

    #[test]
    fn filter_on_wildcard() {
        let input = vec!["2.0.0".to_string(), "1.0.1".to_string(), "1.0.20".to_string(), "1.1.0".to_string()];
        let output = vec![Version::parse("1.0.1").unwrap(), Version::parse("1.0.20").unwrap()];

        assert_eq!(filter_and_sort(input, Some("1.0.*".to_string())).unwrap(), output);
    }
}
