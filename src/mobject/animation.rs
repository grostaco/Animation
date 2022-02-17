use super::mobject::Mobject;

pub trait Animation<M: Mobject> {
    fn mobject(&self) -> &M;
    fn mobject_mut(&mut self) -> &mut M;
    fn starting_mobject(&self) -> &M;
    fn starting_mobject_mut(&mut self) -> &mut M;
    fn interpolate_submobject(&self, start: &mut M, end: &M, alpha: f32);
    fn get_all_mobjects(&mut self) -> (&mut M, &M);
    fn create_starting_mobject(&self) -> M {
        self.mobject().clone()
    }
    fn begin(&mut self) {
        *self.starting_mobject_mut() = self.create_starting_mobject();
        self.interpolate(0.);
    }
    fn end(&mut self) {
        self.interpolate(1.);
    }
    fn interpolate(&mut self, alpha: f32);
    fn interpolate_mobject(&mut self, alpha: f32) {}
}
