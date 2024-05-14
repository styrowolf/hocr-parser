use roxmltree::Node;

use crate::{parsing::{check_property_name, parse_properties}, spec_definitions::HOCR_ELEMENTS, HOCRParserError, Result};

/// Represents an hOCR element, borrowing its contents from the XML string.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ElementBorrowed<'a> {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub node: roxmltree::Node<'a, 'a>,
    pub element_type: &'a str,
    pub properties: Vec<(&'a str, Vec<&'a str>)>,
    pub lang: Option<&'a str>,
    pub text: Option<&'a str>,
    pub children: Vec<ElementBorrowed<'a>>,
}

impl<'a> ElementBorrowed<'a> {
    /// Create a new [`ElementBorrowed`] instance from an [`roxmltree::Node`].
    pub fn from_node(n: Node<'a, 'a>) -> Result<Self> {
        if !n.is_element() {
            let pos = n.document().text_pos_at(n.range().start);
            return Err(HOCRParserError::NodeIsNotElement(pos));
        }

        let element_type = n.attribute("class").unwrap_or("");

        // check if defined in spec or whether it is implementation specific
        if !(HOCR_ELEMENTS.contains(&element_type) || element_type.starts_with("ocrx_")) {
            let pos = n.document().text_pos_at(n.range().start);
            return Err(HOCRParserError::UnknownElement(pos));
        }

        let prop = n.attribute("title").unwrap_or("");
        let properties = parse_properties(prop);

        for (name, _) in &properties {
            if check_property_name(&name) {
                let pos = n.document().text_pos_at(n.range().start);
                return Err(HOCRParserError::UnknownProperty(pos));
            }
        }

        let lang = n.attribute("lang");
        let text = n.text().unwrap_or("");

        // prevent empty lines of whitespace
        let processsed_text = {
            let all_whitespace = text.chars().all(char::is_whitespace);

            if all_whitespace {
                None
            } else {
                Some(text)
            }
        };

        let children: Vec<_> = n
            .children()
            .filter(Node::is_element)
            .map(ElementBorrowed::from_node)
            .collect();

        if let Some(e) = children.iter().find(|r| r.is_err()) {
            return Err(e.as_ref().unwrap_err().clone());
        }

        Ok(Self {
            node: n,
            element_type,
            properties,
            lang,
            text: processsed_text,
            children: children.into_iter().map(Result::unwrap).collect(),
        })
    }
}

/// Represents an hOCR element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    pub element_type: String,
    pub properties: Vec<(String, Vec<String>)>,
    pub lang: Option<String>,
    pub text: Option<String>,
    pub children: Vec<Element>,
}

impl Element {
    /// Create a new [`Element`] instance from an [`ElementBorrowed`].
    pub fn from_element_borrowed(e: &ElementBorrowed) -> Self {
        Self {
            element_type: e.element_type.to_string(),
            properties: e
                .properties
                .iter()
                .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
                .collect(),
            lang: e.lang.map(|l| l.to_string()),
            text: e.text.map(|t| t.to_string()),
            children: e
                .children
                .iter()
                .map(Element::from_element_borrowed)
                .collect(),
        }
    }

    /// Create a new [`Element`] instance from an [`roxmltree::Node`].
    pub fn from_node(n: Node) -> Result<Self> {
        let e = ElementBorrowed::from_node(n)?;
        Ok(Self::from_element_borrowed(&e))
    }
}
