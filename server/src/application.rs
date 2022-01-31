use serde::Deserialize;

use crate::{
    core::{
        error::Result,
        renderer::{Renderer, TeraRenderer},
        website::{Page, Website},
    },
    utils,
};

/// Configuration
#[derive(Debug, Deserialize)]
struct Configuration {
    path: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            path: Default::default(),
        }
    }
}

/// Core props
#[derive(Debug)]
pub struct Application {
    website: Website,
    renderer: TeraRenderer,
}

impl Default for Application {
    fn default() -> Self {
        let configuration: Configuration = utils::read_toml_file("./jilo.config.toml").unwrap();

        let website = Website::new(&configuration.path);

        let renderer = TeraRenderer::new(&website.get_views());

        Self { website, renderer }
    }
}

impl Application {
    // pub fn generate_site<P>(&self, dest_path: P, renderer: &impl Renderer, pages: &Vec<Page>)
    // where
    //     P: AsRef<Path>,
    // {
    //     for page in pages {
    //         self.generate_page(page);
    //     }
    // }

    pub fn generate_page(&self, page: &Page) -> Result<String> {
        log::info!("{:?}", self.website);
        self.renderer.render(page)
    }
}
