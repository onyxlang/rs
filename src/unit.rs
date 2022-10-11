use std::{
    cell::RefCell,
    io,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{
    ast, codegen::Codegen, dst, parser, program::Program, resolve::Resolve, scope::Scope, Panic,
};

pub struct Unit {
    pub program: Weak<RefCell<Program>>,
    pub path: PathBuf,
    ast: Option<ast::Module>,
    dst: Option<dst::Module>,
}

impl Scope for Unit {
    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn find(&self, id: &str) -> Option<Rc<dst::VarDecl>> {
        self.dst.as_ref().and_then(|dst| dst.find(id))
    }
}

impl Unit {
    pub fn new(program: Weak<RefCell<Program>>, path: PathBuf) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            program,
            path,
            ast: None,
            dst: None,
        }))
    }

    pub fn parse(&mut self) -> Result<(), Panic> {
        if self.ast.is_some() {
            return Ok(()); // Already parsed
        }

        let source = std::fs::read_to_string(&self.path)
            .unwrap_or_else(|_| panic!("Failed to read {}", self.path.display()));

        let result = parser::parse(&self.path.to_string_lossy(), source.as_str())?;

        self.ast = Some(result);
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), Panic> {
        if self.dst.is_some() {
            return Ok(()); // Already resolved
        }

        self.parse()?;

        let result = self.ast.as_ref().unwrap().resolve(self)?;

        self.dst = Some(result);
        Ok(())
    }

    pub fn codegen(&self, w: &mut dyn std::io::Write) -> io::Result<()> {
        self.dst.as_ref().expect("Unit not resolved").codegen(w)
    }

    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.path.to_string_lossy().as_bytes());
        hasher.finalize().to_hex().to_string()[..8].to_string()
    }
}
