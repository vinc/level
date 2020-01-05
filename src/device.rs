pub trait Device {
    fn name(&self) -> String;
    fn level(&self) -> u64;
    fn set_level(&self, level: u64);
}
