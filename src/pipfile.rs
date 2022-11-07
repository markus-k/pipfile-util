use toml_edit::{Document, Item, Value};

#[derive(Debug)]
pub struct EditablePipfile {
    document: Document,
}

impl EditablePipfile {
    pub fn from_str(s: &str) -> Result<Self, toml_edit::TomlError> {
        let document = s.parse::<Document>()?;

        Ok(Self { document })
    }

    pub fn iter_deps(&mut self) -> Result<DepIterator, toml_edit::TomlError> {
        let packages = self.document["packages"].as_table_mut().unwrap();

        Ok(DepIterator::new(packages.iter_mut()))
    }
}

pub struct DepIterator<'a> {
    package_iter: toml_edit::IterMut<'a>,
}

impl<'a> DepIterator<'a> {
    fn new(package_iter: toml_edit::IterMut<'a>) -> Self {
        Self { package_iter }
    }
}

#[derive(Debug)]
pub struct DepItem<'a> {
    value: &'a mut Item,
}

impl<'a> DepItem<'a> {
    fn set(&mut self, s: &str) {
        match self.value {
            Item::Value(Value::String(_version_str)) => {
                *self.value = toml_edit::value(s.to_owned());
            }
            Item::Value(Value::InlineTable(dep_info)) if dep_info.get("version").is_some() => {
                dep_info["version"] =
                    toml_edit::Value::String(toml_edit::Formatted::new(s.to_owned()));
            }
            _ => {
                unimplemented!("I do not understand your Pipfile, please create a bug report.")
            }
        }
    }
}

impl<'a> Iterator for DepIterator<'a> {
    type Item = DepItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let package = self.package_iter.next()?.1;

        Some(DepItem { value: package })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pipfile() {
        const PIPFILE: &'static str = include_str!("../testdata/Pipfile");

        let mut pipfile = EditablePipfile::from_str(PIPFILE).unwrap();
        dbg!(&pipfile);
        let iter = pipfile.iter_deps().unwrap();

        for mut item in iter {
            item.set("test");
        }

        dbg!(&pipfile.document.to_string());

        //assert!(false);
    }
}
