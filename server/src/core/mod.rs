use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{self, Deserialize};
use toml::value::Table;

use crate::utils;

/// Errors
#[derive(Debug)]
pub enum CoreError {
    RenderError(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Jilo - Core Error : ")?;
        match self {
            Self::RenderError(io_error) => {
                write!(f, "une erreur est survenue lors du rendu : {}", io_error)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, CoreError>;

/// Core props
#[derive(Debug)]
pub struct Core {
    pub configuration: Configuration,
    pub website: Website,
    pub renderer: TeraRenderer,
}

impl Default for Core {
    fn default() -> Self {
        let configuration: Configuration = utils::read_toml_file("./jilo.config.toml").unwrap();

        let website = Website::new(&configuration.path);

        let renderer = TeraRenderer::new(&configuration.path);

        Self {
            configuration,
            website,
            renderer,
        }
    }
}

#[derive(Debug)]
pub struct Website {
    pages: Pages,
    modules: Modules,
}

impl Website {
    fn new(base_path: &str) -> Self {
        let mut pages: Pages = vec![];
        let mut modules: Modules = vec![];

        pages.parse(&base_path);
        modules.parse(&base_path);

        Self { pages, modules }
    }

    fn generate_site<P>(dest_path: P)
    where
        P: AsRef<Path>,
    {
    }
}

type Pages = Vec<Page>;
type Modules = Vec<Module>;

trait ParseWebsite {
    fn parse(&mut self, path: &str);
}

impl ParseWebsite for Pages {
    fn parse(&mut self, path: &str) {
        if Path::new(path).is_dir() {
            let pattern = format!("{}{}", path, "/pages/**/index.toml");
            for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        let page = Page::new(&path);
                        log::info!("{:?}", page);
                        self.push(page);
                    }
                    _ => (),
                }
            }
        }
    }
}

impl ParseWebsite for Modules {
    fn parse(&mut self, path: &str) {
        if Path::new(path).is_dir() {
            let pattern = format!("{}{}", path, "/modules/**/index.toml");
            for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        let module = Module::new(&path);
                        log::info!("{:?}", module);
                        self.push(module);
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Page {
    view: Option<PathBuf>,
    style: Option<PathBuf>,
    entry: Option<PathBuf>,
}

impl Page {
    fn new(base_path: &PathBuf) -> Self {
        let page: PageConfiguration =
            utils::read_toml_file(&base_path).expect("Toml could not be read");
        let parent = base_path.parent().unwrap_or(Path::new("/"));
        let view = parent
            .join(&page.view)
            .is_file()
            .then(|| parent.join(&page.view));

        let style = parent
            .join(&page.style)
            .is_file()
            .then(|| parent.join(&page.view));

        let entry = parent
            .join(&page.entry)
            .is_file()
            .then(|| parent.join(&page.view));

        Self { view, style, entry }
    }
}

#[derive(Debug, Default, Deserialize)]
struct PageConfiguration {
    #[serde(default = "default_view_path")]
    view: String,

    #[serde(default = "default_style_path")]
    style: String,

    #[serde(default = "default_javascript_path")]
    entry: String,

    data: Table,
}

fn default_view_path() -> String {
    "view.html".to_string()
}

fn default_style_path() -> String {
    "style.scss".to_string()
}

fn default_javascript_path() -> String {
    "index.js".to_string()
}

#[derive(Debug, Default, Deserialize)]
struct ModuleConfiguration {
    view: String,
    style: String,
    entry: String,
    data: Table,
}

#[derive(Debug)]
struct Module {
    view: Option<PathBuf>,
    style: Option<PathBuf>,
    entry: Option<PathBuf>,
}

impl Module {
    fn new(base_path: &PathBuf) -> Self {
        let page: ModuleConfiguration =
            utils::read_toml_file(&base_path).expect("Toml could not be read");
        let parent = base_path.parent().unwrap_or(Path::new("/"));
        let view = parent
            .join(&page.view)
            .is_file()
            .then(|| parent.join(&page.view));

        let style = parent
            .join(&page.style)
            .is_file()
            .then(|| parent.join(&page.view));

        let entry = parent
            .join(&page.entry)
            .is_file()
            .then(|| parent.join(&page.view));

        Self { view, style, entry }
    }
}

/// Configuration
#[derive(Debug, Deserialize)]
pub struct Configuration {
    path: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            path: Default::default(),
        }
    }
}

/// Renderer
pub trait Renderer {
    type Context;
    fn render(&self, name: &str, context: &Self::Context) -> Result<String>;
}

#[derive(Debug)]
pub struct TeraRenderer {
    tera: tera::Tera,
}

impl TeraRenderer {
    fn new(path: &str) -> Self {
        let mut tera = tera::Tera::default();

        // log::info!("is {} a directory ?", path);
        // if Path::new(path).is_dir() {
        //     let pattern = format!("{}{}", path, "/**/index.toml");
        //     log::info!("Pattern is {}", pattern);
        //     for path in glob::glob(&pattern).expect("Failed to read glob pattern") {
        //         // tera.add_template_file(path, Some("base"))
        //     }
        // }

        TeraRenderer { tera }
    }
}

impl Default for TeraRenderer {
    fn default() -> Self {
        let mut tera = tera::Tera::default();
        tera.add_template_file("../site/pages/base/view.html", Some("base"))
            .unwrap();
        tera.add_template_file("../site/pages/product/view.html", Some("product"))
            .unwrap();

        Self { tera }
    }
}

impl Renderer for TeraRenderer {
    type Context = tera::Context;

    fn render(&self, name: &str, c: &Self::Context) -> Result<String> {
        match self.tera.render(name, &c) {
            Ok(html) => Ok(html),
            Err(e) => Err(CoreError::RenderError(e.to_string())),
        }
    }
}
