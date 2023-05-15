use crate::ffi;

/// An enum representing the status of a SCIP optimization run.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// The solving status is not yet known.
    Unknown,
    /// The user interrupted the solving process (by pressing CTRL-C).
    UserInterrupt,
    /// The solving process was interrupted because the node limit was reached.
    NodeLimit,
    /// The solving process was interrupted because the total node limit was reached (incl. restarts).
    TotalNodeLimit,
    /// The solving process was interrupted because the stalling node limit was reached (no improvement w.r.t. primal bound).
    StallNodeLimit,
    /// The solving process was interrupted because the time limit was reached.
    TimeLimit,
    /// The solving process was interrupted because the memory limit was reached.
    MemoryLimit,
    /// The solving process was interrupted because the gap limit was reached.
    GapLimit,
    /// The solving process was interrupted because the solution limit was reached.
    SolutionLimit,
    /// The solving process was interrupted because the solution improvement limit was reached.
    BestSolutionLimit,
    /// The solving process was interrupted because the restart limit was reached.
    RestartLimit,
    /// The problem was solved to optimality, an optimal solution is available.
    Optimal,
    /// The problem was proven to be infeasible.
    Infeasible,
    /// The problem was proven to be unbounded.
    Unbounded,
    /// The problem was proven to be either infeasible or unbounded.
    Inforunbd,
    /// Status if the process received a SIGTERM signal.
    Terminate,
}

impl From<u32> for Status {
    /// Converts a u32 value to a `Status` enum variant.
    fn from(val: u32) -> Self {
        match val {
            ffi::SCIP_Status_SCIP_STATUS_UNKNOWN => Status::Unknown,
            ffi::SCIP_Status_SCIP_STATUS_USERINTERRUPT => Status::UserInterrupt,
            ffi::SCIP_Status_SCIP_STATUS_NODELIMIT => Status::NodeLimit,
            ffi::SCIP_Status_SCIP_STATUS_TOTALNODELIMIT => Status::TotalNodeLimit,
            ffi::SCIP_Status_SCIP_STATUS_STALLNODELIMIT => Status::StallNodeLimit,
            ffi::SCIP_Status_SCIP_STATUS_TIMELIMIT => Status::TimeLimit,
            ffi::SCIP_Status_SCIP_STATUS_MEMLIMIT => Status::MemoryLimit,
            ffi::SCIP_Status_SCIP_STATUS_GAPLIMIT => Status::GapLimit,
            ffi::SCIP_Status_SCIP_STATUS_SOLLIMIT => Status::SolutionLimit,
            ffi::SCIP_Status_SCIP_STATUS_BESTSOLLIMIT => Status::BestSolutionLimit,
            ffi::SCIP_Status_SCIP_STATUS_RESTARTLIMIT => Status::RestartLimit,
            ffi::SCIP_Status_SCIP_STATUS_OPTIMAL => Status::Optimal,
            ffi::SCIP_Status_SCIP_STATUS_INFEASIBLE => Status::Infeasible,
            ffi::SCIP_Status_SCIP_STATUS_UNBOUNDED => Status::Unbounded,
            ffi::SCIP_Status_SCIP_STATUS_INFORUNBD => Status::Inforunbd,
            ffi::SCIP_Status_SCIP_STATUS_TERMINATE => Status::Terminate,
            _ => panic!("Unknown SCIP status {val:?}"),
        }
    }
}