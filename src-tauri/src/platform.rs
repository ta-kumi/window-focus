pub mod windows;

pub trait MainModule {
    fn initialize(&mut self);
    fn finialize(&mut self);

    fn focus_on(&self);
    fn focus_off(&self);
}
