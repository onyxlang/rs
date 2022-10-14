use crate::{unit::Unit, Panic};
use std::{
    cell::RefCell,
    fs::create_dir_all,
    io::{self, Write},
    path::PathBuf,
    process::Command,
    rc::Rc,
};

pub struct Program {
    cache_path: PathBuf,
    cache_dir_ensured: bool,
    units: Vec<Rc<RefCell<Unit>>>,
}

impl Program {
    pub fn new(cache_path: PathBuf) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            cache_path,
            cache_dir_ensured: false,
            units: Vec::new(),
        }))
    }

    /// Resolve a unit at `path`.
    /// Adds the unit to the program if it doesn't exist.
    /// The very first resolved unit becomes the program entry unit.
    pub fn resolve(this: Rc<RefCell<Self>>, path: PathBuf) -> Result<Rc<RefCell<Unit>>, Panic> {
        for unit in &this.as_ref().borrow().units {
            if unit.as_ref().borrow().path == path {
                return Ok(Rc::clone(unit));
            }
        }

        // Ensure the cache directory exists only once.
        if !this.as_ref().borrow().cache_dir_ensured {
            create_dir_all(&this.as_ref().borrow().cache_path).unwrap();
            this.as_ref().borrow_mut().cache_dir_ensured = true;
        }

        let unit = Unit::new(Rc::downgrade(&this), path);
        Unit::resolve(unit.clone())?;
        this.borrow_mut().units.push(unit.clone());

        Ok(unit)
    }

    pub fn run(
        this: Rc<RefCell<Self>>,
        input_path: PathBuf,
        zig_path: PathBuf,
    ) -> Result<(), Panic> {
        let entry_unit_lowered_path = Self::lower(Rc::clone(&this), input_path)?;

        let mut zig_cache_path = this.as_ref().borrow().cache_path.clone();
        zig_cache_path.push("./zig");

        let mut cmd = Command::new(zig_path.as_path());
        cmd.args([
            "run",
            entry_unit_lowered_path.as_path().to_str().unwrap(),
            "-lc",
            "--cache-dir",
            zig_cache_path.as_path().to_str().unwrap(),
        ]);
        dbg!(&cmd);

        let output = cmd.output().unwrap();

        if !(output.status.success()) {
            println!("Zig exited with status {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            panic!("Failed to run {}", entry_unit_lowered_path.display());
        }

        Ok(())
    }

    pub fn compile(
        this: Rc<RefCell<Self>>,
        input_path: PathBuf,
        output_path: PathBuf,
        zig_path: PathBuf,
    ) -> Result<(), Panic> {
        let entry_unit_lowered_path = Self::lower(Rc::clone(&this), input_path)?;

        let mut zig_cache_path = this.as_ref().borrow().cache_path.clone();
        zig_cache_path.push("./zig");

        let mut cmd = Command::new(zig_path.as_path());
        cmd.args([
            "build-exe",
            entry_unit_lowered_path.as_path().to_str().unwrap(),
            "-lc",
            "--cache-dir",
            zig_cache_path.as_path().to_str().unwrap(),
            ("-femit-bin=".to_string() + output_path.as_path().to_str().unwrap()).as_str(),
        ]);
        dbg!(&cmd);

        let output = cmd.output().unwrap();

        if !(output.status.success()) {
            println!("Zig exited with status {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            panic!("Failed to compile {}", entry_unit_lowered_path.display());
        }

        Ok(())
    }

    /// Lower the whole program, starting with the entry unit.
    /// Returns the entry point lowered path.
    fn lower(this: Rc<RefCell<Self>>, entry_path: PathBuf) -> Result<PathBuf, Panic> {
        let entry = Self::resolve(Rc::clone(&this), entry_path)?;

        let path = entry
            .as_ref()
            .borrow_mut()
            .lower(this.as_ref().borrow().cache_path.clone());

        Ok(path)
    }
}
