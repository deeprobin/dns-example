use arrayvec::{ArrayString, CapacityError};

// See RFC 2181, section 11. Name syntax
const MAX_LABEL_LENGTH: usize = 63;

#[derive(Debug, Clone)]
pub struct DNSLabel(pub ArrayString<MAX_LABEL_LENGTH>);

impl DNSLabel {
    pub fn new(s: &str) -> Result<Self, CapacityError<&str>> {
        let array_string = ArrayString::from(s)?;
        Ok(DNSLabel(array_string))
    }
}

impl From<String> for DNSLabel {
    fn from(value: String) -> Self {
        let mut array_string = ArrayString::<MAX_LABEL_LENGTH>::new();
        array_string.push_str(&value);

        DNSLabel(array_string)
    }
}

impl From<DNSLabel> for String {
    fn from(value: DNSLabel) -> Self {
        value.0.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_label_creation() {
        let label = DNSLabel::new("www.example.com");
        assert!(label.is_ok());

        let label_str = &label.unwrap().0;
        assert_eq!(label_str, "www.example.com");
    }

    #[test]
    fn test_dns_label_too_long() {
        let long_string: String = std::iter::repeat("a").take(MAX_LABEL_LENGTH + 1).collect();
        let label = DNSLabel::new(&long_string);
        assert!(label.is_err());
    }

    #[test]
    fn test_dns_label_exact_length() {
        let exact_length_string: String = std::iter::repeat("a").take(MAX_LABEL_LENGTH).collect();
        let label = DNSLabel::new(&exact_length_string);
        assert!(label.is_ok());

        let binding = label.unwrap();
        let label_str = binding.0.as_str();
        assert_eq!(label_str, &exact_length_string);
    }
}
