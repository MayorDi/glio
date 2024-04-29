use crate::traits::Bindable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VAO {
    pub id: u32,
    pub is_bound: bool,
}

impl VAO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {
            id,
            is_bound: false,
        }
    }
}

impl Bindable for VAO {
    fn bind(&mut self) {
        if self.is_bound {
            return;
        }

        unsafe {
            gl::BindVertexArray(self.id);
        }

        self.is_bound = true;
    }

    fn unbind(&mut self) {
        if !self.is_bound {
            return;
        }

        unsafe {
            gl::BindVertexArray(0);
        }

        self.is_bound = false;
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
