use crate::shader::Shader;
use crate::traits::{AttachShaders, Compilable, Linkable, Status};
use gl::types::{GLchar, GLint};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub id: u32,
    pub is_linked: bool,
    is_used: bool,
    shaders: Vec<Shader>,
}

impl Program {
    pub fn new() -> Self {
        unsafe {
            Self {
                id: gl::CreateProgram(),
                is_linked: false,
                is_used: false,
                shaders: vec![],
            }
        }
    }

    pub fn push_shader(&mut self, shader: Shader) {
        self.shaders.push(shader);
    }

    pub fn employ(&mut self) {
        unsafe {
            if !self.is_used {
                self.is_used = true;
                gl::UseProgram(self.id);
            }
        }
    }
}

impl AttachShaders for Program {
    fn attach(&mut self) -> Result<(), String> {
        for shader in self.shaders.iter_mut() {
            shader.compile()?;
            unsafe {
                gl::AttachShader(self.id, shader.id);
            }
        }

        Ok(())
    }
}

impl Linkable for Program {
    fn link(&mut self) -> Result<(), String> {
        if self.is_linked {
            return Ok(());
        }

        self.attach()?;

        unsafe {
            gl::LinkProgram(self.id);
            self.status()?;
        }

        self.is_linked = true;
        Ok(())
    }
}

impl Status for Program {
    fn status(&self) -> Result<(), String> {
        unsafe {
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
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

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
