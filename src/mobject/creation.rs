use super::{animation::Animation, mobject::Mobject};

pub trait ShowPartial<M: Mobject>: Animation<M> {
    fn _get_bounds(&self, alpha: f32) -> (f32, f32);
}

pub struct Create<M: Mobject> {
    starting_mobject: M,
    mobject: M,
}

impl<M: Mobject> ShowPartial<M> for Create<M> {
    fn _get_bounds(&self, alpha: f32) -> (f32, f32) {
        (0., alpha)
    }
}

impl<M: Mobject> Animation<M> for Create<M> {
    fn mobject(&self) -> &M {
        &self.mobject
    }

    fn mobject_mut(&mut self) -> &mut M {
        &mut self.mobject
    }

    fn starting_mobject(&self) -> &M {
        &self.starting_mobject
    }
    fn starting_mobject_mut(&mut self) -> &mut M {
        &mut self.starting_mobject
    }

    fn get_all_mobjects(&mut self) -> (&mut M, &M) {
        (&mut self.starting_mobject, &self.mobject)
    }

    fn interpolate_submobject(&self, start: &mut M, end: &M, alpha: f32) {}

    fn interpolate(&mut self, alpha: f32) {
        let (submobject, starting_submobject) = self.get_all_mobjects();
        let mut submobject = submobject.clone();
        let starting_submobject = starting_submobject.clone();

        let (start, end) = self._get_bounds(alpha);
        *self.mobject_mut() = submobject
            .pointwise_become_partial(starting_submobject.clone(), start, end)
            .clone();
    }
}

impl<M: Mobject> Create<M> {
    pub fn new(starting_mobject: M, mobject: M) -> Self {
        Self {
            starting_mobject,
            mobject,
        }
    }
}
