use thiserror::Error;

/// hOCR parsing error variants.
#[derive(Error, Debug, Clone)]
pub enum HOCRParserError {
    /// Unknown hOCR element.
    #[error("Unknown element found in hOCR file at {0}: elements either must be defined in the spec or start with 'ocrx_' prefix")]
    UnknownElement(roxmltree::TextPos),
    /// Unknown hOCR property.
    #[error("Unknown property found in hOCR file at {0}: properties either must be defined in the spec or start with 'x_' prefix")]
    UnknownProperty(roxmltree::TextPos),
    /// Cannot construct hOCR element from node: it is not of type Element.
    #[error("Cannot construct hOCR Element from node: it is not of type Element at {0}")]
    NodeIsNotElement(roxmltree::TextPos),
    /// No `<head>` element found in hOCR file.
    #[error("No <head> element found in hOCR file")]
    NoHeadElement,
    /// No `<body>` element found in hOCR file.
    #[error("No <body> element found in hOCR file")]
    NoBodyElement,
    /// No OCR system found in hOCR file metadata.
    #[error("No OCR system found in hOCR file metadata; invalid hOCR according to spec")]
    NoOCRSystem,
    /// No OCR capabilities found in hOCR file metadata.
    #[error("No OCR capabilities found in hOCR file metadata; invalid hOCR according to spec")]
    NoOCRCapabilities,
    /// XML parse error.
    #[error("roxmltree error: {0}")]
    XMLParseError(#[from] roxmltree::Error),
}

/// A `Result` type alias using `HOCRParserError` instances as the error variant.
pub type Result<T, E = HOCRParserError> = std::result::Result<T, E>;
