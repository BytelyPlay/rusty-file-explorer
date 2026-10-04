use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;

pub type InnerVecModel<T> = Rc<VecModel<T>>;

pub fn get_inner_vec_model<T: 'static>(
    model_rc: &ModelRc<T>
) -> Option<InnerVecModel<T>> {
    model_rc
        .as_any()
        .downcast_ref::<InnerVecModel<T>>()
        .cloned()
}
pub fn set_inner_model_to_vec_model<
    T: std::clone::Clone + 'static
>(
    model_rc: &mut ModelRc<T>
) -> InnerVecModel<T> {
    let files_vec: Vec<T> = model_rc.iter()
        .collect();
    let rc_files_vec_model = Rc::new(
        VecModel::from(
            files_vec
        )
    );
    *model_rc = ModelRc::new(
        rc_files_vec_model.clone()
    );
    rc_files_vec_model.clone()
}