//! # Overview
//! 
//! A parser for the [hOCR](http://kba.github.io/hocr-spec/1.2/) format, "an open standard for representing document layout analysis and OCR results as a subset of HTML."
//! 
//! ## Design 
//! 
//! This parser uses [`roxmltree`](https://github.com/RazrFalcon/roxmltree) to parse the XHTML. It simplifies provides easy access to the hOCR data embedded through the `HOCR` and `Element` structs, as well as their "borrowed" counterparts to prevent allocating for property names.
//! 
//! The parser does not validate if the file adheres to the hOCR specification. It checks required metadata and validity of hOCR element and property names but does not check property values.

mod element;
mod error;
mod iter;
mod hocr;
mod parsing;
/// Contains the element and property names defined in the hOCR specification.
pub mod spec_definitions;

pub use error::{HOCRParserError, Result};
pub use hocr::{HOCR, HOCRBorrowed};
pub use element::{Element, ElementBorrowed};

pub use roxmltree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quoted_properties() {
        let property = "image \"Screenshot 2024-05-12 at 14.21.17.png\"; bbox 0 0 796 1314; ppageno 0; scan_res 144 144";
        let res = parsing::parse_properties(&property);
        println!("{:?}", res);
        let image_prop = res.iter().find(|(n, _)| *n == "image").unwrap();
        assert_eq!(image_prop.1, vec!["Screenshot 2024-05-12 at 14.21.17.png"]);
    }

    #[test]
    fn parse_multiple_quoted_properties() {
        let property = r#"x_source abc def "/gfs/cc/clean/012345678911" "17" abc def "Screenshot 2024-05-12 at 14.21.17.png""#;
        let res = parsing::parse_properties(&property);
        println!("{:?}", res);
        let prop = res.iter().find(|(n, _)| *n == "x_source").unwrap();
        assert_eq!(
            prop.1,
            vec![
                "abc",
                "def",
                "/gfs/cc/clean/012345678911",
                "17",
                "abc",
                "def",
                "Screenshot 2024-05-12 at 14.21.17.png"
            ]
        );
    }

    #[test]
    fn parse_empty_property() {
        let property = "";
        let res = parsing::parse_properties(&property);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn parse_just_whitespace_property() {
        let property = "     \n  \t  \n  \t  \n";
        let res = parsing::parse_properties(&property);
        assert_eq!(res.len(), 0);
    }
}
