use std::{
    cell::RefCell,
    io::Result,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{ast, codegen::Codegen, dst, parser, program::Program, resolve::Resolve, scope::Scope};

pub struct Unit {
    pub program: Weak<RefCell<Program>>,
    pub path: PathBuf,
    ast: Option<ast::Module>,
    dst: Option<dst::Module>,
}

impl Scope for Unit {
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

    pub fn parse(&mut self) {
        if self.ast.is_some() {
            return;
        }

        let source = std::fs::read_to_string(&self.path)
            .unwrap_or_else(|_| panic!("Failed to read {}", self.path.display()));

        let result = parser::onyx_parser::start(source.as_str());
        if let Err(err) = result {
            panic!("Failed to parse {}: {}", self.path.display(), err);
        }

        self.ast = Some(result.ok().unwrap());
    }

    pub fn resolve(&mut self) {
        if self.dst.is_some() {
            return;
        }

        self.parse();

        let rc = self.program.upgrade().unwrap();
        let borrow = rc.as_ref().borrow();

        let result = self.ast.as_ref().unwrap().resolve(&*borrow);
        if let Err(err) = result {
            panic!("Failed to resolve {}: {}", self.path.display(), err);
        }

        self.dst = Some(result.ok().unwrap());
    }

    pub fn codegen(&self, w: &mut dyn std::io::Write) -> Result<()> {
        self.dst.as_ref().expect("Unit not resolved").codegen(w)
    }

    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.path.to_string_lossy().as_bytes());
        hasher.finalize().to_hex().to_string()[..8].to_string()
    }
}
