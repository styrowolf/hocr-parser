use roxmltree::{Node, ParsingOptions};

use crate::error::Result;
use crate::{element::Element, element::ElementBorrowed, HOCRParserError};

/// Represents a hOCR file, borrowing its contents from the XML string.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HOCRBorrowed<'input> {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub document: &'input roxmltree::Document<'input>,
    pub system: &'input str,
    pub capabilities: Vec<&'input str>,
    pub number_of_pages: Option<u32>,
    pub langs: Option<Vec<&'input str>>,
    pub scripts: Option<Vec<&'input str>>,
    pub elements: Vec<ElementBorrowed<'input>>,
}

impl<'input> HOCRBorrowed<'input> {
    /// Create a new [`HOCRBorrowed`] instance from a [`roxmltree::Document`].
    pub fn new_from_document(document: &'input roxmltree::Document<'input>) -> Result<Self> {
        let head = document
            .root_element()
            .children()
            .find(|e| e.tag_name().name() == "head")
            .ok_or(HOCRParserError::NoHeadElement)?;

        let metadata: Vec<_> = head
            .children()
            .filter_map(|e| {
                if e.tag_name().name() == "meta" && e.has_attribute("name") {
                    let name = e.attribute("name")?;
                    let content = e.attribute("content")?;
                    Some((name, content))
                } else {
                    None
                }
            })
            .collect();

        let system = metadata
            .iter()
            .find(|(name, _)| *name == "ocr-system")
            .map(|(_, content)| content)
            .ok_or(HOCRParserError::NoOCRSystem)?;

        let capabilities = metadata
            .iter()
            .find(|(name, _)| *name == "ocr-capabilities")
            .map(|(_, content)| content.split_whitespace().collect())
            .ok_or(HOCRParserError::NoOCRCapabilities)?;

        let number_of_pages = metadata
            .iter()
            .find(|(name, _)| *name == "ocr-number-of-pages")
            .map(|(_, content)| content.parse().ok())
            .flatten();

        let langs = metadata
            .iter()
            .find(|(name, _)| *name == "ocr-langs")
            .map(|(_, content)| content.split_whitespace().collect());

        let scripts = metadata
            .iter()
            .find(|(name, _)| *name == "scripts")
            .map(|(_, content)| content.split_whitespace().collect());

        let body = document
            .root_element()
            .children()
            .find(|e| e.tag_name().name() == "body")
            .ok_or(HOCRParserError::NoBodyElement)?;

        let elements: Vec<_> = body
            .children()
            .filter(Node::is_element)
            .map(ElementBorrowed::from_node)
            .collect();

        if let Some(e) = elements.iter().find(|r| r.is_err()) {
            return Err(e.as_ref().unwrap_err().clone());
        }

        Ok(Self {
            document: document,
            system,
            capabilities,
            number_of_pages,
            langs,
            scripts,
            elements: elements.into_iter().map(Result::unwrap).collect(),
        })
    }
}

/// Represents a hOCR file.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HOCR {
    pub system: String,
    pub capabilities: Vec<String>,
    pub number_of_pages: Option<u32>,
    pub langs: Option<Vec<String>>,
    pub scripts: Option<Vec<String>>,
    pub elements: Vec<Element>,
}

impl HOCR {
    /// Create a new [`HOCR`] instance from a string containing hOCR XML.
    pub fn from_str(xml_str: &str) -> Result<Self> {
        let mut options = ParsingOptions::default();
        options.allow_dtd = true;

        let doc = roxmltree::Document::parse_with_options(&xml_str, options)?;

        let hocr = HOCRBorrowed::new_from_document(&doc)?;
        Ok(Self::from_hocr_borrowed(hocr))
    }

    /// Create a new [`HOCR`] instance from a [`HOCRBorrowed`].
    pub fn from_hocr_borrowed(hocr: HOCRBorrowed) -> Self {
        Self {
            system: hocr.system.to_string(),
            capabilities: hocr.capabilities.iter().map(|s| s.to_string()).collect(),
            number_of_pages: hocr.number_of_pages,
            langs: hocr
                .langs
                .map(|l| l.iter().map(|s| s.to_string()).collect()),
            scripts: hocr
                .scripts
                .map(|s| s.iter().map(|s| s.to_string()).collect()),
            elements: hocr
                .elements
                .iter()
                .map(Element::from_element_borrowed)
                .collect(),
        }
    }
}