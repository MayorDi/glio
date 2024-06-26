use crate::traits::Bindable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VAO {
    pub id: u32,
}

impl VAO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {
            id
        }
    }
}

impl Bindable for VAO {
    fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
