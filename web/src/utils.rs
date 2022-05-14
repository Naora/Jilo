use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Link {
    pub(crate) rel: String,
    pub(crate) href: String,
}

#[derive(Serialize)]
pub(crate) struct Response<T>
where
    T: Serialize,
{
    pub(crate) data: Option<T>,
    pub(crate) links: Vec<Link>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub(crate) fn success(data: T) -> Self {
        Self {
            data: Some(data),
            links: vec![],
        }
    }

    pub(crate) fn add_link<I>(&mut self, rel: I, href: I)
    where
        I: Into<String>,
    {
        let rel = rel.into();
        let href = href.into();
        self.links.push(Link { rel, href })
    }
}
