use slint::{Model, ModelError, ModelRc, VecModel};
use std::rc::Rc;

// TODO: This should probably be in a separate "slint-helpers" library.
pub struct ModelRcAccessor<T: Clone + 'static> {
    model_rc_getter: fn() -> ModelRc<T>,
    model_rc_setter: fn(model_rc: ModelRc<T>)
}

impl<T: Clone + 'static> ModelRcAccessor<T> {
    pub fn push(&self, value: T) {
        let model = (self.model_rc_getter)();

        match model.push_row(value.clone()) {
            Ok(ok) => {},
            Err(err) => {
                if err.eq(&ModelError::out_of_bounds(0)) {
                    panic!(
                        "This panic should NEVER be called. \
                        push was called in ModelRcAccessor struct and the out_of_bounds error was thrown: {}",
                        err
                    )
                } else if err.eq(
                    &ModelError::unsupported(
                        &VecModel::from(vec![0])
                    )
                ) {
                    self.make_mutable();

                    model.push_row(value)
                        .expect(format!("A ModelError was thrown after calling push after a \
                        ModelError of type unsupported was thrown and it was made mutable: {}", err)
                            .as_str())
                }
            }
        }
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
                self.handle_remove_i_model_error(err);
                model.remove_row(index)
                    .expect(
                        "Couldn't remove an index after handle_model_error was called \
                        and returned successfully."
                    )
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
    /// This handles a model_error.
    /// If it is an out_of_bounds error, it will panic.
    ///
    /// If it's an unsupported operation error, it will call make_mutable and return.
    /// In the case stated above, you'd have to run the operation again.
    ///
    /// If it is another error, it will also panic. Because at the time of writing there is no other error.
    /// If it returns, you should call the operation again.
    fn handle_remove_i_model_error(&self, err: ModelError) {
        // This a very stupid way of figuring out the error, but I have no other choice.

        if err == ModelError::out_of_bounds(0) {
            panic!("The function remove_i in ModelRcAccessor was called with an invalid index:\
            {}", err)
        } else if err == ModelError::unsupported(
            &VecModel::from(
                vec![0]
            )
        ) {
            self.make_mutable();
        } else {
            panic!("The error in ModelRcAccessor is neither the out_of_bounds error type\
             nor the unsupported error type, \
                    and those are all the available error types at the time of writing.\
                    This is a bug in the ModelRcAccessor helper struct (therefore the program)\
                    or a bug in Slint: {}", err)
        }
    }
}