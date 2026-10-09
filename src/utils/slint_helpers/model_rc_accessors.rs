use slint::{Model, ModelRc, VecModel};
use std::iter::once;
use std::rc::Rc;

// TODO: This should probably be in a separate "slint-helpers" library.

// This is supposed to wrap a ModelRc<T> in a way, that it behaves almost exactly like a Vec<T>.
pub struct ModelRcAccessor<T: Clone + 'static> {
    model_rc_getter: Box<dyn Fn() -> ModelRc<T>>,
    model_rc_setter: Box<dyn Fn(ModelRc<T>)>
}

impl<T: Clone + 'static> ModelRcAccessor<T> {
    pub fn new(
        model_rc_getter: Box<dyn Fn() -> ModelRc<T>>,
        model_rc_setter: Box<dyn Fn(ModelRc<T>)>
    ) -> Self {
        Self {
            model_rc_getter,
            model_rc_setter
        }
    }
    pub fn push(&self, value: T) {
        self.set_vec(
            self.iter()
                .chain(once(value))
                .collect()
        )
    }
    pub fn get(&self, index: usize) -> Option<T> {
        (self.model_rc_getter)()
            .row_data(index)
    }
    /// This panics if you try to remove an index that doesn't exist.
    /// Please be aware of that.
    pub fn remove_i(&self, index: usize) {
        let model = (self.model_rc_getter)();

        if model.row_count() < index + 1 {
            panic!(
                "remove_i was \
                called with the index: \
                {}, but there are only \
                {} rows (max index: {})",
                index,

                model.row_count(),
                model.row_count() - 1
            )
        }

        self.set_vec(
            self.iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, v)| v)
                .collect()
        )
    }

    /// Panics if the ModelRc<T> doesn't contain value.
    pub fn remove_v(&self, value: T)
    where
    T: PartialEq
    {
        let mut index: usize = 0;

        let array_contains = self.iter()
            .enumerate()
            .any(
                |(i, v)| {
                    if v == value {
                        index = i;
                        return true;
                    }
                    false
                }
            );

        if !array_contains {
            panic!("remove_v called with a value that isn't in the model.")
        } else {
            self.remove_i(index);
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = T> {
        (self.model_rc_getter)()
            .iter()
            .collect::<Vec<T>>()
            .into_iter()
    }
    pub fn set_vec(&self, vec: Vec<T>) {
        (self.model_rc_setter)(
            ModelRc::new(
                Rc::new(
                    VecModel::from(
                        vec
                    )
                )
            )
        )
    }
}