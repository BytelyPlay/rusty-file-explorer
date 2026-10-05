use std::any::Any;
use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;

pub type InnerVecModel<T> = Rc<VecModel<T>>;

pub fn get_inner_vec_model<T: 'static>(
    model_rc: &ModelRc<T>
) -> Option<InnerVecModel<T>> {
    let as_any = model_rc.as_any();
    let r = as_any
        .downcast_ref::<InnerVecModel<T>>()
        .cloned();
    println!("For debugging DEL ME");
    r
}
pub fn create_model_copy_with_vec_model<
    T: std::clone::Clone + 'static
>(
    model_rc: &ModelRc<T>
) -> ModelRc<T> {
    let files_vec: Vec<T> = model_rc.iter()
        .collect();
    let rc_files_vec_model = Rc::new(
        VecModel::from(
            files_vec
        )
    );
    ModelRc::new(
        rc_files_vec_model.clone()
    )
}