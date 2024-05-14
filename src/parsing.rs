use crate::spec_definitions::HOCR_PROPERTIES;

pub(crate) fn parse_properties(prop: &str) -> Vec<(&str, Vec<&str>)> {
    prop.split(';')
        .filter_map(|p| {
            let mut iter = p.split_whitespace();
            let property_name = iter.next()?;

            let values = p.trim().strip_prefix(property_name)?;

            let property_values: Vec<&str> = values
                .split('"')
                .flat_map(|s| {
                    let is_just_whitespace = s.chars().all(char::is_whitespace);

                    if is_just_whitespace {
                        return vec![];
                    }

                    if s.trim() != s {
                        return s.split_whitespace().collect();
                    }

                    vec![s]
                })
                .collect();

            Some((property_name, property_values))
        })
        .collect()
}

pub(crate) fn check_property_name(prop: &str) -> bool {
    HOCR_PROPERTIES.contains(&prop) || prop.starts_with("x_")
}