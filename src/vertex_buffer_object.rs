use crate::traits::{Bindable, Load, WriteData};
use gl::types::GLenum;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VBO<T> {
    pub id: u32,
    pub target: Target,
    pub type_draw: TypeDraw,
    pub data: Option<T>,
    pub is_bound: bool,
}

impl<T> VBO<T> {
    pub fn new(target: Target, type_draw: TypeDraw) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self {
            id,
            target,
            type_draw,
            data: None,
            is_bound: false,
        }
    }
}

impl<T> WriteData<T> for VBO<T> {
    fn write(&mut self, data: T) {
        self.data = Some(data);
    }
}

impl<T> Load for VBO<T> {
    type Output = Self;
    fn load(self) -> std::io::Result<Self::Output> {
        match &self.data {
            Some(data) => unsafe {
                gl::BufferData(
                    self.target.into(),
                    std::mem::size_of_val(data) as isize,
                    std::mem::transmute(data),
                    self.type_draw.into(),
                )
            },

            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "The data is not exist",
                ))
            }
        }

        Ok(self)
    }
}

impl<T> Bindable for VBO<T> {
    fn bind(&mut self) {
        if self.is_bound {
            return;
        }

        unsafe {
            gl::BindBuffer(self.target.into(), self.id);
        }

        self.is_bound = true;
    }

    fn unbind(&mut self) {
        if !self.is_bound {
            return;
        }

        unsafe {
            gl::BindBuffer(self.target.into(), 0);
        }

        self.is_bound = false;
    }
}

impl<T> Drop for VBO<T> {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Target {
    ArrayBuffer,
    ElementArrayBuffer,
    TextureBuffer,
    UniformBuffer,
}

impl Into<GLenum> for Target {
    fn into(self) -> GLenum {
        match self {
            Self::ArrayBuffer => gl::ARRAY_BUFFER,
            Self::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            Self::TextureBuffer => gl::TEXTURE_BUFFER,
            Self::UniformBuffer => gl::UNIFORM_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeDraw {
    GlStaticDraw,
    GlDynamicDraw,
    GlStreamDraw,
}

impl Into<GLenum> for TypeDraw {
    fn into(self) -> GLenum {
        match self {
            TypeDraw::GlStaticDraw => gl::STATIC_DRAW,
            TypeDraw::GlDynamicDraw => gl::DYNAMIC_DRAW,
            TypeDraw::GlStreamDraw => gl::STREAM_DRAW,
        }
    }
}
