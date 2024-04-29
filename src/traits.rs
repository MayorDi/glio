use std::path::PathBuf;

pub type Result = std::result::Result<(), String>;

pub trait Load {
    type Output;
    fn load(self) -> std::io::Result<Self::Output>;
}

pub trait LoadFromFile {
    type Output;
    
    fn load(self, path: PathBuf) -> std::io::Result<Self::Output>;
}

pub trait WriteData<T> {
    fn write(&mut self, data: T);
}

pub trait Status {
    fn status(&self) -> Result;
}

pub trait Compilable {
    fn compile(&mut self) -> Result;
}

pub trait Linkable{
    fn link(&mut self) -> Result;
}

pub trait AttachShaders {
    fn attach(&mut self) -> Result;
}

pub trait Bindable {
    fn bind(&mut self);
    
    fn unbind(&mut self);
}
