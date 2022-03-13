pub trait Storage {
    fn load(&self);
    fn persist(&self);
}
