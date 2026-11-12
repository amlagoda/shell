use crate::rule::Comprasion;

pub fn complete_to_variants(starts_with: &str, variants: &Vec<&str>) -> Option<Completion> {
    if variants.is_empty() {
        return None;
    }

    if starts_with.is_empty() {
        let variants = variants.iter().map(|r| r.to_string()).collect();
        return Some(Completion::from_variants(variants));
    }

    let mut found = vec![];

    for variant in variants {
        if Comprasion::PatternStartsWithNotEqual(variant.to_string()).assert(starts_with) {
            found.push(variant.to_string());
        }
    }

    let len = found.len();

    if len == 0 {
        return None;
    }

    let short = found.iter().min_by_key(|r| r.len()).unwrap();

    if len == 1 {
        let selected = format!("{} ", short.replacen(starts_with, "", 1));
        return Some(Completion::from_selected(selected));
    }

    let variants = found.iter().map(|r| r.as_str()).collect();

    if let Some(prefixed) = get_short_prefixed_variant(starts_with, variants) {
        let selected = prefixed.replacen(starts_with, "", 1);
        Some(Completion::from_selected(selected))
    } else {
        Some(Completion::from_variants(found))
    }
}

pub fn get_short_prefixed_variant(current: &str, mut variants: Vec<&str>) -> Option<String> {
    if variants.is_empty() {
        return None;
    }

    if variants.len() == 1 {
        return Some(variants[0].to_string());
    }

    let short = if current.is_empty() {
        variants.iter().min_by_key(|r| r.len()).unwrap()
    } else {
        variants = variants
            .iter()
            .filter(|r| r.trim_end_matches("/") != current)
            .copied()
            .collect();

        variants.iter().min_by_key(|r| r.len()).unwrap()
    };

    let is_chain = variants
        .iter()
        .all(|r| Comprasion::PatternStartsWith(r.to_string()).assert(short.trim_end_matches("/")));

    if !is_chain {
        return None;
    }

    Some(short.to_string())
}

#[derive(Debug, PartialEq)]
pub struct Completion {
    selected: Option<String>,
    variants: Option<Vec<String>>,
}

impl Completion {
    pub fn from_selected(selected: String) -> Completion {
        Completion {
            selected: Some(selected),
            variants: None,
        }
    }

    pub fn from_variants(variants: Vec<String>) -> Completion {
        Completion {
            selected: None,
            variants: Some(variants),
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected.is_some()
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn get_variants(&self) -> Option<Vec<&str>> {
        self.variants
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_complete_to_variants() -> Result<(), Error> {
        let variants = vec![];
        assert!(complete_to_variants("", &variants).is_none());

        let variants = vec!["f", "b"];
        let variants = complete_to_variants("", &variants).unwrap();
        let variants = variants.get_variants().unwrap();
        assert_eq!(vec!["f", "b"], variants);

        let variants = vec!["f", "b"];
        assert!(complete_to_variants("c", &variants).is_none());

        let variants = vec!["fo", "b"];
        let selected = complete_to_variants("f", &variants).unwrap();
        let selected = selected.get_selected().unwrap();
        assert_eq!("o ", selected);

        let variants = vec!["fo", "fy"];
        let variants = complete_to_variants("f", &variants).unwrap();
        let variants = variants.get_variants().unwrap();
        assert_eq!(vec!["fo", "fy"], variants);

        let variants = vec!["fo", "foo"];
        let selected = complete_to_variants("f", &variants).unwrap();
        let selected = selected.get_selected().unwrap();
        assert_eq!("o", selected);

        Ok(())
    }

    #[test]
    fn test_get_short_prefixed_variant() -> Result<(), Error> {
        let variants = vec![];
        assert!(get_short_prefixed_variant("", variants).is_none());

        let variants = vec![];
        assert!(get_short_prefixed_variant("f", variants).is_none());

        let variants = vec!["f"];
        assert_eq!("f", get_short_prefixed_variant("", variants).unwrap());

        let variants = vec!["f"];
        assert_eq!("f", get_short_prefixed_variant("f", variants).unwrap());

        let variants = vec!["f", "fo", "b"];
        assert!(get_short_prefixed_variant("", variants).is_none());

        let variants = vec!["f", "fo", "foo"];
        assert_eq!("f", get_short_prefixed_variant("", variants).unwrap());

        let variants = vec!["f", "fo", "foo", "fy"];
        assert!(get_short_prefixed_variant("f", variants).is_none());

        let variants = vec!["f", "fo", "foo"];
        assert_eq!("fo", get_short_prefixed_variant("f", variants).unwrap());

        let variants = vec!["fbzn.txt", "fbz/", "fb/"];
        assert_eq!("fb/", get_short_prefixed_variant("f", variants).unwrap());

        Ok(())
    }
}
