mod module;

use std::{fmt, path::Path};

use serde::{self, Deserialize};

use crate::utils;

use self::module::Module;

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
    // pub renderer: TeraRenderer,
}

impl Default for Core {
    fn default() -> Self {
        let configuration: Configuration = utils::read_toml_file("./jilo.config.toml").unwrap();

        let website = Website::new(&configuration.path);

        // let renderer = TeraRenderer::new(&configuration.path);

        Self {
            configuration,
            website,
            // renderer,
        }
    }
}

#[derive(Debug)]
pub struct Website {
    pages: Vec<Module>,
    modules: Vec<Module>,
}

impl Website {
    fn new(base_path: &str) -> Self {
        let mut pages = vec![];
        let mut modules = vec![];

        if Path::new(base_path).is_dir() {
            pages.parse_module(&format!("{}{}", base_path, "/pages/**/index.toml"));
            modules.parse_module(&format!("{}{}", base_path, "/modules/**/index.toml"));
        }
        Self { pages, modules }
    }

    fn generate_site<P>(dest_path: P)
    where
        P: AsRef<Path>,
    {
    }
}

trait ParseModuleForPattern {
    fn parse_module(&mut self, pattern: &str);
}

impl ParseModuleForPattern for Vec<Module> {
    fn parse_module(&mut self, pattern: &str) {
        for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let page = Module::new(&path);
                    log::info!("{:?}", page);
                    self.push(page);
                }
                _ => (),
            }
        }
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

// Renderer
// pub trait Renderer {
//     type Context;
//     fn render(&self, name: &str, context: &Self::Context) -> Result<String>;
// }

// #[derive(Debug)]
// pub struct TeraRenderer {
//     tera: tera::Tera,
// }

// impl TeraRenderer {
//     fn new(path: &str) -> Self {
//         let mut tera = tera::Tera::default();

//         // log::info!("is {} a directory ?", path);
//         // if Path::new(path).is_dir() {
//         //     let pattern = format!("{}{}", path, "/**/index.toml");
//         //     log::info!("Pattern is {}", pattern);
//         //     for path in glob::glob(&pattern).expect("Failed to read glob pattern") {
//         //         // tera.add_template_file(path, Some("base"))
//         //     }
//         // }

//         TeraRenderer { tera }
//     }
// }

// impl Default for TeraRenderer {
//     fn default() -> Self {
//         let mut tera = tera::Tera::default();
//         tera.add_template_file("../site/pages/base/view.html", Some("base"))
//             .unwrap();
//         tera.add_template_file("../site/pages/product/view.html", Some("product"))
//             .unwrap();

//         Self { tera }
//     }
// }

// impl Renderer for TeraRenderer {
//     type Context = tera::Context;

//     fn render(&self, name: &str, c: &Self::Context) -> Result<String> {
//         match self.tera.render(name, &c) {
//             Ok(html) => Ok(html),
//             Err(e) => Err(CoreError::RenderError(e.to_string())),
//         }
//     }
// }
