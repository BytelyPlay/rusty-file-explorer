use slint::{Model, ModelError, ModelRc, VecModel};
use std::rc::Rc;

pub struct ModelRcAccessor<T: Clone + 'static> {
    model_rc_getter: fn() -> ModelRc<T>,
    model_rc_setter: fn(model_rc: ModelRc<T>)
}

impl<T: Clone + 'static> ModelRcAccessor<T> {
    pub fn push(&self, value: T) {
        (self.model_rc_getter)()
            .push_row()
    }
    pub fn get(&self, index: usize) -> Option<T> {
        (self.model_rc_getter)()
            .row_data(index)
    }
    /// This panics if you try to remove an index that doesn't exist.
    /// Please be aware of that.
    pub fn remove_i(&self, index: usize) {
        let model = (self.model_rc_getter)();
        let result = model
            .remove_row(index);
        match result {
            Ok(ok) => {},
            Err(err) => {
                // This a very stupid way of figuring out the error, but I have no other choice.
                if err == ModelError::out_of_bounds(0) {
                    panic!("remove_i called with an invalid index.")
                } else if err == ModelError::unsupported(
                    &VecModel::from(
                        vec![0]
                    )
                ) {
                   self.make_mutable();
                   model.remove_row(index)
                        .expect("After confirming that a remove_i call was legal, there was still an error. This is never supposed to happen.");
                } else {
                    panic!("The error in ModelRcAccessor is neither out_of_bounds nor unsupported, \
                    and those are all the available error types at the time of writing.")
                }
            }
        }
    }
    pub fn remove_v(&self, value: T)
    where
    T: PartialEq {
        let vec: Vec<(usize, T)> = self.iter()
            .enumerate()
            .filter(|e| value.eq(&e.1))
            .collect();

        if vec.len() > 1 {
            panic!("An element was duplicated when calling remove_v.")
        } else if vec.len() < 1 {
            panic!("The element that we tried to remove using remove_v doesn't exist.")
        } else {
            self.remove_i(vec[0].0);
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = T> {
        (self.model_rc_getter)()
            .iter()
            .collect::<Vec<T>>()
            .into_iter()
    }
    fn make_mutable(&self) {
        let model = (self.model_rc_getter)();
        (self.model_rc_setter)(
            ModelRc::new(
                Rc::new(
                    VecModel::from(
                        model.iter().collect::<Vec<T>>()
                    )
                )
            )
        )
    }
    /// This handles a model_error, if it is an out_of_bounds error, it will panic,
    /// if it's an unsupported operation error, it will call make_mutable.
    ///
    fn handle_model_error(&self) {

    }
}