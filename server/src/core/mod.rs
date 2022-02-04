pub mod error;
pub mod module;
mod persistance;
mod renderer;
mod utils;
pub mod website;

use std::{fs, io::Write, path::Path};

use serde::Deserialize;

use self::{
    error::Result,
    persistance::{locale::Locale, Persist},
    renderer::{tera::TeraRenderer, Renderer},
    website::{Page, Website},
};

/// Configuration
#[derive(Debug, Deserialize)]
struct Configuration {
    path: String,
}

/// Core props
#[derive(Debug)]
pub struct Application {
    website: Website,
    renderer: TeraRenderer,
    persistance: Locale,
}

impl Default for Application {
    fn default() -> Self {
        let configuration: Configuration = utils::read_toml_file("./jilo.config.toml").unwrap();

        let website = Website::new(&configuration.path);

        let renderer = TeraRenderer::new(&website.get_views());

        let persistance = Locale::new("./tests/site/data");

        Self {
            website,
            renderer,
            persistance,
        }
    }
}

impl Application {
    pub fn generate_site<P>(&self, dest_path: P)
    where
        P: AsRef<Path>,
    {
        let dest_path = dest_path.as_ref();
        fs::create_dir_all(dest_path); // TODO handle Result
        for (path, page) in &self.persistance.get_all_pages() {
            let html = self.generate_page(&page).unwrap(); // Todo handle Result
            let path = path.to_owned() + ".html";
            let path = dest_path.join(&path);
            log::info!("{:?}", path);
            let mut dest_html_file = fs::File::create(path).unwrap(); // Todo handle Result
            dest_html_file.write_all(html.as_bytes()).unwrap(); // Todo handle Result
        }
    }

    pub fn generate_page(&self, page: &Page) -> Result<String> {
        log::info!("{:?}", self.website);
        self.renderer.render(page)
    }
}
