//! # russcip
//! Safe Rust interface for SCIP.
//!
//! # Example
//! Model and solve an integer program.
//! ```rust
//! use russcip::model::Model;
//! use russcip::model::ObjSense;
//! use russcip::status::Status;
//! use russcip::variable::VarType;
//! use crate::russcip::model::ModelWithProblem;
//!
//! fn main() -> Result<(), russcip::retcode::Retcode> {
//!   // Create model
//!   let mut model = Model::new()
//!   .hide_output()
//!   .include_default_plugins()
//!   .create_prob("test")
//!   .set_obj_sense(ObjSense::Maximize);
//!
//!   // Add variables
//!   let x1_id = model.add_var(0., f64::INFINITY, 3., "x1", VarType::Integer);
//!   let x2_id = model.add_var(0., f64::INFINITY, 4., "x2", VarType::Integer);
//!
//!   // Add constraints
//!   model.add_cons(&[x1_id, x2_id], &[2., 1.], -f64::INFINITY, 100., "c1");
//!   model.add_cons(&[x1_id, x2_id], &[1., 2.], -f64::INFINITY, 80., "c2");
//!
//!   let solved_model = model.solve();
//!
//!   let status = solved_model.get_status();
//!   println!("Solved with status {:?}", status);
//!
//!   let obj_val = solved_model.get_obj_val();
//!   println!("Objective value: {}", obj_val);
//!
//!   let sol = solved_model.get_best_sol().expect("No solution found");
//!   let vars = solved_model.get_vars();
//!
//!   for var in vars {
//!       println!("{} = {}", &var.get_name(), sol.get_var_val(&var));
//!   }
//!   Ok(())
//! }

extern crate doc_comment;
doc_comment::doctest!("../README.md");

pub use scip_sys as ffi;
pub mod constraint;
pub mod model;
pub mod retcode;
pub mod solution;
pub mod status;
pub mod variable;

#[macro_export]
macro_rules! scip_call {
    ($res:expr) => {
        let res = unsafe { $res };
        let retcode = $crate::retcode::Retcode::from(res);
        if retcode != $crate::retcode::Retcode::Okay {
            return Err(retcode);
        }
    };
}

#[macro_export]
macro_rules! scip_call_panic {
    ($res:expr) => {
        let res = unsafe { $res };
        let retcode = $crate::retcode::Retcode::from(res);
        if retcode != $crate::retcode::Retcode::Okay {
            panic!("SCIP call failed with retcode {:?}", retcode);
        }
    };
}

#[macro_export]
macro_rules! scip_call_expect {
    ($res:expr, $msg:expr) => {
        let res = unsafe { $res };
        let retcode = $crate::retcode::Retcode::from(res);
        if retcode != $crate::retcode::Retcode::Okay {
            panic!("{} - SCIP call failed with retcode {:?}", $msg, retcode);
        }
    };
}
