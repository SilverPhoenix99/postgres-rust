pub trait Named {
    fn name(self: Self) -> &'static str;
}
