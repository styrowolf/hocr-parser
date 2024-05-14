/// An array of all hOCR element types.
pub const HOCR_ELEMENTS: [&str; 39] = [
    // Typesetting elements
    "ocr_page",
    "ocr_column",
    "ocr_carea",
    "ocr_line",
    "ocr_separator",
    "ocr_noise",
    // Float elements
    "ocr_float",
    "ocr_textfloat",
    "ocr_textimage",
    "ocr_image",
    "ocr_linedrawing",
    "ocr_photo",
    "ocr_header",
    "ocr_footer",
    "ocr_pageno",
    "ocr_table",
    // Logical elements
    "ocr_document",
    "ocr_title",
    "ocr_author",
    "ocr_abstract",
    "ocr_part",
    "ocr_chapter",
    "ocr_section",
    "ocr_subsection",
    "ocr_subsubsection",
    "ocr_display",
    "ocr_blockquote",
    "ocr_par",
    "ocr_linear",
    "ocr_caption",
    // Inline elements
    "ocr_glyph",
    "ocr_glyphs",
    "ocr_dropcap",
    "ocr_math",
    "ocr_chem",
    "ocr_cinfo",
    // OCR Engine-specific elements
    "ocrx_block",
    "ocrx_line",
    "ocrx_word",
];

/// An array of all hOCR properties.
pub const HOCR_PROPERTIES: [&str; 21] = [
    "baseline",
    "bbox",
    "cflow",
    "cuts",
    "hardbreak",
    "image",
    "imagemd5",
    "lpageno",
    "ppageno",
    "nlp",
    "order",
    "poly",
    "scan_res",
    "textangle",
    "x_bboxes",
    "x_font",
    "x_fsize",
    "x_confs",
    "x_scanner",
    "x_source",
    "x_wconf",
];

/// All hOCR properties as variables for auto-complete.
pub mod properties {
    pub const BASELINE: &str = "baseline";
    pub const BBOX: &str = "bbox";
    pub const CFLOW: &str = "cflow";
    pub const CUTS: &str = "cuts";
    pub const HARDBREAK: &str = "hardbreak";
    pub const IMAGE: &str = "image";
    pub const IMAGEMD5: &str = "imagemd5";
    pub const LPAGENO: &str = "lpageno";
    pub const PPAGENO: &str = "ppageno";
    pub const NLP: &str = "nlp";
    pub const ORDER: &str = "order";
    pub const POLY: &str = "poly";
    pub const SCAN_RES: &str = "scan_res";
    pub const TEXTANGLE: &str = "textangle";
    pub const X_BBOXES: &str = "x_bboxes";
    pub const X_FONT: &str = "x_font";
    pub const X_FSIZE: &str = "x_fsize";
    pub const X_CONFS: &str = "x_confs";
    pub const X_SCANNER: &str = "x_scanner";
    pub const X_SOURCE: &str = "x_source";
    pub const X_WCONF: &str = "x_wconf";
}

/// All hOCR element types as variables for auto-complete.
pub mod elements {
    pub use float::*;
    pub use inline::*;
    pub use logical::*;
    pub use ocr_engine_specific::*;
    pub use typesetting::*;

    pub mod typesetting {
        pub const OCR_PAGE: &str = "ocr_page";
        pub const OCR_COLUMN: &str = "ocr_column";
        pub const OCR_CAREA: &str = "ocr_carea";
        pub const OCR_LINE: &str = "ocr_line";
        pub const OCR_SEPERATOR: &str = "ocr_seperator";
        pub const OCR_NOISE: &str = "ocr_noise";
    }
    
    pub mod float {
        pub const OCR_FLOAT: &str = "ocr_float";
        pub const OCR_TEXTFLOAT: &str = "ocr_textfloat";
        pub const OCR_TEXTIMAGE: &str = "ocr_textimage";
        pub const OCR_IMAGE: &str = "ocr_image";
        pub const OCR_LINEDRAWING: &str = "ocr_linedrawing";
        pub const OCR_PHOTO: &str = "ocr_photo";
        pub const OCR_HEADER: &str = "ocr_header";
        pub const OCR_FOOTER: &str = "ocr_footer";
        pub const OCR_PAGENO: &str = "ocr_pageno";
        pub const OCR_TABLE: &str = "ocr_table";
    }
    
    pub mod logical {
        pub const OCR_DOCUMENT: &str = "ocr_document";
        pub const OCR_TITLE: &str = "ocr_title";
        pub const OCR_AUTHOR: &str = "ocr_author";
        pub const OCR_ABSTRACT: &str = "ocr_abstract";
        pub const OCR_PART: &str = "ocr_part";
        pub const OCR_CHAPTER: &str = "ocr_chapter";
        pub const OCR_SECTION: &str = "ocr_section";
        pub const OCR_SUBSECTION: &str = "ocr_subsection";
        pub const OCR_SUBSUBSECTION: &str = "ocr_subsubsection";
        pub const OCR_DISPLAY: &str = "ocr_display";
        pub const OCR_BLOCKQUOTE: &str = "ocr_blockquote";
        pub const OCR_PAR: &str = "ocr_par";
        pub const OCR_LINEAR: &str = "ocr_linear";
        pub const OCR_CAPTION: &str = "ocr_caption";
    }
    
    pub mod inline {
        pub const OCR_GLYPH: &str = "ocr_glyph";
        pub const OCR_GLYPHS: &str = "ocr_glyphs";
        pub const OCR_DROPCAP: &str = "ocr_dropcap";
        pub const OCR_MATH: &str = "ocr_math";
        pub const OCR_CHEM: &str = "ocr_chem";
        pub const OCR_CINFO: &str = "ocr_cinfo";
    }
    
    pub mod ocr_engine_specific {
        pub const OCRX_BLOCK: &str = "ocrx_block";
        pub const OCRX_LINE: &str = "ocrx_line";
        pub const OCRX_WORD: &str = "ocrx_word";
    }    
}