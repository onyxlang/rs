use crate::{ast, dst, lower::Lowerable, parser, program::Program, Panic};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

pub struct Unit {
    pub program: Weak<RefCell<Program>>,
    pub path: PathBuf,
    ast: Option<ast::Mod>,
    pub dst: Option<dst::Mod>,
    pub lowered_path: Option<PathBuf>,
    pub dependencies: Vec<Weak<RefCell<Unit>>>,
}

impl Unit {
    pub fn new(program: Weak<RefCell<Program>>, path: PathBuf) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            program,
            path,
            ast: None,
            dst: None,
            lowered_path: None,
            dependencies: Vec::new(),
        }))
    }

    pub fn parse(&mut self) -> Result<(), Panic> {
        if self.ast.is_some() {
            return Ok(()); // Already parsed
        }

        let result = match self.path.to_str() {
            Some("builtin") => parser::parse(
                "../lang/builtin.nx".into(),
                include_str!("../lang/builtin.nx"),
            )?,

            Some("builtin/bool") => parser::parse(
                "../lang/builtin/bool.nx".into(),
                include_str!("../lang/builtin/bool.nx"),
            )?,

            _ => {
                let source = std::fs::read_to_string(&self.path);

                if source.is_err() {
                    return Err(Panic::new(
                        format!(
                            "Failed to read file at \"{}\": {}",
                            self.path.display(),
                            source.err().unwrap()
                        ),
                        None,
                    ));
                }

                parser::parse(self.path.clone(), &source.unwrap())?
            }
        };

        self.ast = Some(result);
        Ok(())
    }

    pub fn resolve(this: Rc<RefCell<Self>>) -> Result<(), Panic> {
        if this.borrow().dst.is_some() {
            return Ok(()); // Already resolved
        }

        this.borrow_mut().parse()?;

        let ast = this.borrow_mut().ast.take().unwrap();
        let result = ast.resolve(Rc::downgrade(&this))?;

        this.borrow_mut().dst = Some(result);

        Ok(())
    }

    pub fn lower(&mut self, cache_path: PathBuf) -> PathBuf {
        if self.lowered_path.is_some() {
            return self.lowered_path.as_ref().unwrap().to_path_buf(); // Already lowered
        }

        // Lower all dependencies first.
        for dependency in &self.dependencies {
            dependency
                .upgrade()
                .unwrap()
                .borrow_mut()
                .lower(cache_path.clone());
        }

        let lowering_path = cache_path.join(self.hash()).with_extension("zig");
        println!(
            "Lowering \"{}\" to \"{}\"...",
            self.path.display(),
            lowering_path.display()
        );

        let mut file = std::fs::File::create(&lowering_path).unwrap();
        let result = self
            .dst
            .as_ref()
            .expect("Unit must be resolved")
            .lower(&mut file);
        if result.is_err() {
            panic!(
                "Failed to lower \"{}\" to \"{}\": {}",
                self.path.display(),
                lowering_path.display(),
                result.err().unwrap()
            );
        }

        self.lowered_path = Some(lowering_path.clone());
        lowering_path
    }

    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.path.to_string_lossy().as_bytes());
        hasher.finalize().to_hex().to_string()[..8].to_string()
    }
}
