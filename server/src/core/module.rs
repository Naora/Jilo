use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::{
    error::{Error, ErrorKind, Result},
    utils,
};

fn get_default_view_path() -> String {
    "view.html".to_string()
}

fn get_default_style_path() -> String {
    "style.scss".to_string()
}

fn get_default_javascript_path() -> String {
    "index.js".to_string()
}

#[derive(Debug, Deserialize)]
struct ModuleFile {
    #[serde(default = "get_default_view_path")]
    view: String,
    #[serde(default = "get_default_style_path")]
    style: String,
    #[serde(default = "get_default_javascript_path")]
    entry: String,
    fields: Option<Vec<Field>>, // Todo use data to fetch props from modules
    areas: Option<Vec<Area>>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    name: String,
    r#type: FieldTypes,
}

#[derive(Deserialize, Debug)]
enum FieldTypes {
    String,
    Integer,
    Boolean,
}

#[derive(Debug, Default)]
pub struct Module {
    pub index: PathBuf,
    pub view: Option<PathBuf>,
    pub style: Option<PathBuf>,
    pub entry: Option<PathBuf>,
    pub fields: Vec<Field>,
    pub areas: Vec<Area>,
}

#[derive(Deserialize, Debug)]
pub struct Area {
    pub name: String,
    #[serde(default)]
    pub modules: Vec<String>,
}

impl Module {
    pub fn new<P>(base_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let base_path = PathBuf::from(base_path.as_ref());
        let mut module = Self::default();
        module.index = base_path.to_owned();

        let module_file: ModuleFile =
            utils::read_toml_file(&base_path).expect("Toml could not be read");
        module.load(module_file);

        module
    }

    fn load<'a>(&'a mut self, module_file: ModuleFile) {
        let parent = self.index.parent().unwrap_or(Path::new("/"));

        self.view = is_file_present(&parent, &module_file.view);
        self.style = is_file_present(&parent, &module_file.style);
        self.entry = is_file_present(&parent, &module_file.entry);

        self.fields = match module_file.fields {
            Some(fields) => fields,
            None => vec![],
        };

        self.areas = match module_file.areas {
            Some(areas) => areas,
            None => vec![],
        }
    }

    pub fn get_module_name(&self) -> Result<String> {
        let parent = match self.index.parent() {
            Some(parent) => parent,
            None => return Err(Error::new(ErrorKind::Module)),
        };

        if parent.is_dir() {
            if let Some(directory) = parent.file_name() {
                let lossy = directory.to_string_lossy();
                return Ok(lossy.into_owned());
            }
        }
        Err(Error::new(ErrorKind::Module))
    }
}

fn is_file_present<P>(dir: P, part: &str) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = dir.as_ref().join(part);
    path.is_file().then(|| path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_a_module() {
        let module = Module::new("./tests/site/modules/module_1/index.toml");
        assert_eq!(
            module.index,
            PathBuf::from("./tests/site/modules/module_1/index.toml")
        );
        assert_eq!(module.view, None);
        assert_eq!(module.entry, None);
        assert_eq!(module.style, None);

        let module = Module::new("./tests/site/modules/module_2/index.toml");
        assert_eq!(
            module.index,
            PathBuf::from("./tests/site/modules/module_2/index.toml")
        );
        assert_eq!(
            module.view,
            Some(PathBuf::from("./tests/site/modules/module_2/view.html"))
        );
        assert_eq!(
            module.entry,
            Some(PathBuf::from("./tests/site/modules/module_2/index.js"))
        );
        assert_eq!(
            module.style,
            Some(PathBuf::from("./tests/site/modules/module_2/style.scss"))
        );
    }

    #[test]
    fn test_if_file_is_present() {
        let file_exists = is_file_present("./", "jilo.config.toml");
        assert!(file_exists.is_some());

        let file_does_not_exist = is_file_present("./", "hahah.tomli");
        assert!(file_does_not_exist.is_none());
    }

    #[test]
    fn retrieve_module_name() {
        let module = Module::new("./tests/site/modules/module_1/index.toml");
        let name = module.get_module_name().unwrap();
        assert_eq!(name, "module_1".to_string());

        let module = Module::new("./tests/site/modules/module_2/index.toml");
        let name = module.get_module_name().unwrap();
        assert_eq!(name, "module_2".to_string());
    }
}
