use crate::traits::{Status, WriteData};
use crate::traits::{Compilable, LoadFromFile};
use gl::types::{GLchar, GLenum, GLint};
use std::ffi::CString;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Shader {
    pub id: u32,
    pub type_shader: TypeShader,
    pub is_compile: bool,
    src: String,
}

impl Shader {
    pub fn new(type_shader: TypeShader) -> Self {
        unsafe {
            Self {
                id: gl::CreateShader(type_shader.into()),
                type_shader,
                is_compile: false,
                src: String::new(),
            }
        }
    }
}

impl WriteData<String> for Shader {
    fn write(&mut self, data: String) {
        self.src = data;
    }
}

impl LoadFromFile for Shader {
    type Output = Self;

    fn load(mut self, path: PathBuf) -> std::io::Result<Self::Output> {
        let file = std::fs::read_to_string(path)?;
        self.src = file;

        Ok(self)
    }
}

impl Compilable for Shader {
    fn compile(&mut self) -> Result<(), String> {
        if self.is_compile {
            return Ok(());
        }

        unsafe {
            let src = CString::new(self.src.clone()).unwrap();
            gl::ShaderSource(self.id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }

        if self.status().is_err() {
            return Err(self.status().err().unwrap());
        }

        self.is_compile = true;
        Ok(())
    }
}

impl Status for Shader {
    fn status(&self) -> Result<(), String> {
        unsafe {
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                return Err(String::from_utf8(buf).unwrap());
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeShader {
    Vertex,
    Geometry,
    Fragment,
}

impl Into<GLenum> for TypeShader {
    fn into(self) -> GLenum {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Geometry => gl::GEOMETRY_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
