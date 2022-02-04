pub mod locale;

use super::{error::Result, website};

pub trait Persist {
    fn load(&self) -> Self;
    fn save(&self) -> Result<()>;

    fn get_all_pages(&self) -> website::Pages;
}
