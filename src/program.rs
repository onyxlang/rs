use std::{
    cell::RefCell,
    fs::create_dir_all,
    io::{self, Write},
    path::PathBuf,
    process::Command,
    rc::Rc,
};

use crate::{dst, scope::Scope, unit::Unit};

pub struct Program {
    cache_path: PathBuf,
    zig_path: PathBuf,
    entry: Rc<RefCell<Unit>>,
    units: Vec<Rc<RefCell<Unit>>>,
}

impl Program {
    pub fn new(entry_path: PathBuf, cache_path: PathBuf, zig_path: PathBuf) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|program| {
            let entry = Unit::new(program.clone(), entry_path);
            let units = vec![entry.clone()];

            RefCell::new(Self {
                cache_path,
                zig_path,
                entry,
                units,
            })
        })
    }

    pub fn add_unit(this: Rc<RefCell<Self>>, path: PathBuf) {
        this.borrow_mut()
            .units
            .push(Unit::new(Rc::downgrade(&this), path));
    }

    pub fn run(this: Rc<RefCell<Self>>) {
        let borrow = this.borrow();
        let mut unit = borrow.entry.borrow_mut();

        unit.parse();
        unit.resolve();

        let cache_path = borrow.cache_path.clone();
        create_dir_all(cache_path.as_path()).unwrap();

        let unit_path = borrow.unit_cache_path(&unit);
        println!("Writing {}...", unit_path.display());

        let mut file = std::fs::File::create(&unit_path).unwrap();
        unit.codegen(&mut file);

        let mut zig_cache_path = cache_path.clone();
        zig_cache_path.push("./zig");

        let mut cmd = Command::new(borrow.zig_path.as_path());
        cmd.args([
            "run",
            unit_path.as_path().to_str().unwrap(),
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
            panic!("Failed to run {}", unit_path.display());
        }
    }

    fn unit_cache_path(&self, unit: &Unit) -> PathBuf {
        let mut path = self.cache_path.clone();
        path.push("./");
        path.push(unit.hash());
        path.with_extension("zig")
    }
}

impl Scope for Program {
    fn find(&self, _id: &str) -> Option<Rc<dst::VarDecl>> {
        None
    }
}
