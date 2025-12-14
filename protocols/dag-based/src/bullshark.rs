use std::{collections::VecDeque, rc::Rc};

use simulator::*;

use crate::dag_utils::{RoundBasedDAG, VertexPtr};

#[derive(Clone)]
enum BullsharkMessage {
    Vertex(VertexPtr),
}

impl Message for BullsharkMessage {
    fn VirtualSize(&self) -> usize {
        todo!()
    }
}

struct Vertex {}

struct Bullshark {
    self_id: ProcessId,
    dag: RoundBasedDAG,
    round: usize,
    buffer: VecDeque<VertexPtr>,
}

impl Bullshark {
    fn New() -> Self {
        Self {
            self_id: 0,
            dag: RoundBasedDAG::New(),
            round: 1,
            buffer: VecDeque::new(),
        }
    }
}

impl ProcessHandle<BullsharkMessage> for Bullshark {
    fn Bootstrap(
        &mut self,
        configuration: Configuration,
        outgoing: &mut simulator::OutgoingMessages<BullsharkMessage>,
    ) {
        self.dag.Init(configuration.proc_num);
        self.self_id = configuration.assigned_id;
    }

    fn OnMessage(
        &mut self,
        from: ProcessId,
        message: BullsharkMessage,
        outgoing: &mut simulator::OutgoingMessages<BullsharkMessage>,
    ) {
        todo!();
    }
}

impl Bullshark {
    fn TryAdvanceRound(&mut self) {
        todo!()
    }
}

fn main() {
    todo!()
}
