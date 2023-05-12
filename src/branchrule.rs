use std::rc::Rc;
use crate::ffi;
use crate::variable::Variable;

pub trait BranchRule {
    fn execute(&mut self, candidates: Vec<BranchingCandidate>) -> BranchingResult;
}

#[derive(Debug, Clone)]
pub enum BranchingResult {
    DidNotRun,
    /// Initiate branching on the given candidate
    BranchOn(BranchingCandidate),
    /// Current node is detected to be infeasible and can be cut off
    CutOff,
    /// A custom branching scheme is implemented
    CustomBranching,
    /// A cutting plane is added
    Separated,
    /// Reduced the domain of a variable such that the current LP solution becomes infeasible
    ReduceDom,
    /// A constraint was added
    ConsAdded,
}

impl From<BranchingResult> for u32 {
    fn from(val: BranchingResult) -> Self {
        match val {
            BranchingResult::DidNotRun => ffi::SCIP_Result_SCIP_DIDNOTRUN,
            BranchingResult::BranchOn(_) => ffi::SCIP_Result_SCIP_BRANCHED,
            BranchingResult::CutOff => ffi::SCIP_Result_SCIP_CUTOFF,
            BranchingResult::CustomBranching => ffi::SCIP_Result_SCIP_BRANCHED,
            BranchingResult::Separated => ffi::SCIP_Result_SCIP_SEPARATED,
            BranchingResult::ReduceDom => ffi::SCIP_Result_SCIP_REDUCEDDOM,
            BranchingResult::ConsAdded => ffi::SCIP_Result_SCIP_CONSADDED,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BranchingCandidate {
    pub var: Rc<Variable>,
    pub lp_sol_val: f64,
    pub frac: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ModelRef, ModelWithProblem, ProblemCreated};
    use crate::{model::Model, status::Status};

    struct PanickingBranchingRule;
    impl BranchRule for PanickingBranchingRule {
        fn execute(&mut self, _candidates: Vec<BranchingCandidate>) -> BranchingResult {
            panic!("Not implemented")
        }
    }

    #[test]
    #[should_panic]
    fn panicking_branchrule() {
        let mut br = PanickingBranchingRule {};

        let model = Model::new()
            .hide_output()
            .include_default_plugins()
            .read_prob("data/test/gen-ip054.mps")
            .unwrap()
            .include_branch_rule("", "", 100000, 1000, 1., &mut br)
            .solve();
    }

    struct FirstChoosingBranchingRule {
        pub chosen: Option<BranchingCandidate>,
    }

    impl BranchRule for FirstChoosingBranchingRule {
        fn execute(&mut self, candidates: Vec<BranchingCandidate>) -> BranchingResult {
            self.chosen = Some(candidates[0].clone());
            BranchingResult::DidNotRun
        }
    }

    #[test]
    fn choosing_first_branching_rule() {
        let mut br = FirstChoosingBranchingRule { chosen: None };

        let model = Model::new()
            .set_longint_param("limits/nodes", 2) // only call brancher once
            .unwrap()
            .hide_output()
            .include_default_plugins()
            .read_prob("data/test/gen-ip054.mps")
            .unwrap()
            .include_branch_rule("", "", 100000, 1000, 1., &mut br);

        let solved = model.solve();
        assert_eq!(solved.get_status(), Status::NodeLimit);
        assert!(br.chosen.is_some());
        let candidate = br.chosen.unwrap();
        assert!(candidate.lp_sol_val.fract() > 0.);
        assert!(candidate.frac > 0. && candidate.frac < 1.);
    }

    struct CuttingOffBranchingRule;

    impl BranchRule for CuttingOffBranchingRule {
        fn execute(&mut self, _candidates: Vec<BranchingCandidate>) -> BranchingResult {
            BranchingResult::CutOff
        }
    }

    #[test]
    fn cutting_off_branching_rule() {
        let mut br = CuttingOffBranchingRule {};

        // create model from miplib instance gen-ip054
        let model = Model::new()
            .hide_output()
            .include_default_plugins()
            .read_prob("data/test/gen-ip054.mps")
            .unwrap()
            .include_branch_rule("", "", 100000, 1000, 1., &mut br)
            .solve();
        assert_eq!(model.get_n_nodes(), 1);
    }

    struct FirstBranchingRule {
        model: ModelRef<Model<ProblemCreated>>,
    }

    impl BranchRule for FirstBranchingRule {
        fn execute(&mut self, candidates: Vec<BranchingCandidate>) -> BranchingResult {
            assert!(self.model.get_n_vars() >= candidates.len());
            BranchingResult::BranchOn(candidates[0].clone())
        }
    }

    #[test]
    fn first_branching_rule() {
        let mut model = Model::new()
            .hide_output()
            .set_longint_param("limits/nodes", 2)
            .unwrap() // only call brancher once
            .include_default_plugins()
            .read_prob("data/test/gen-ip054.mps")
            .unwrap();

        let mut br = FirstBranchingRule {
            model: ModelRef::new(&mut model),
        };
        let mut solved = model
            .include_branch_rule("", "", 100000, 1000, 1., &mut br)
            .solve();

        assert!(solved.get_n_nodes() > 1);
    }
}
