use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse<T> {
    success: bool,
    data: T,
}

impl<T> JsonResponse<T> {
    pub fn new(success: bool, data: T) -> Self {
        JsonResponse { success, data }
    }
}
