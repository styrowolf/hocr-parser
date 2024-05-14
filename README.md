# hocr-parser

A parser for the [hOCR](http://kba.github.io/hocr-spec/1.2/) format, "an open standard for representing document layout analysis and OCR results as a subset of HTML."

## Design 

This parser uses [`roxmltree`](https://github.com/RazrFalcon/roxmltree) to parse the XHTML. It simplifies provides easy access to the hOCR data embedded through the `HOCR` and `Element` structs, as well as their "borrowed" counterparts to prevent allocating for property names.

The parser does not validate if the file adheres to the hOCR specification. It checks required metadata and validity of hOCR element and property names but does not check property values.

## License

Licensed under either of

- [Apache License v2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

