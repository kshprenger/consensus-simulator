// https://arxiv.org/pdf/2201.05677
// https://arxiv.org/pdf/2209.05633

use std::collections::{HashSet, VecDeque};

use simulator::*;

use crate::dag_utils::{RoundBasedDAG, Vertex, VertexPtr};

#[derive(Clone)]
enum BullsharkMessage {
    Vertex(VertexPtr),
}

impl Message for BullsharkMessage {
    fn VirtualSize(&self) -> usize {
        todo!()
    }
}

struct Bullshark {
    self_id: ProcessId,
    proc_num: usize,
    dag: RoundBasedDAG,
    round: usize,
    buffer: HashSet<VertexPtr>,
}

impl Bullshark {
    fn New() -> Self {
        Self {
            self_id: 0,
            proc_num: 0,
            dag: RoundBasedDAG::New(),
            round: 1,
            buffer: HashSet::new(),
        }
    }
}

impl ProcessHandle<BullsharkMessage> for Bullshark {
    fn Bootstrap(
        &mut self,
        configuration: Configuration,
        outgoing: &mut OutgoingMessages<BullsharkMessage>,
    ) {
        self.dag.Init(configuration.proc_num);
        self.self_id = configuration.assigned_id;
        self.proc_num = configuration.proc_num;
        self.TryAdvanceRound(outgoing);
    }

    // DAG construction: part 1
    fn OnMessage(
        &mut self,
        from: ProcessId,
        message: BullsharkMessage,
        outgoing: &mut OutgoingMessages<BullsharkMessage>,
    ) {
        todo!();
    }
}

// Utils
impl Bullshark {
    fn AdversaryThreshold(&self) -> usize {
        (self.proc_num - 1) / 3
    }

    fn QuorumSize(&self) -> usize {
        2 * self.AdversaryThreshold() + 1
    }

    fn CreateVertex(&self, round: usize) -> VertexPtr {
        VertexPtr::new(Vertex {
            round,
            source: self.self_id,
            strong_edges: self.dag[round - 1]
                .iter()
                .flatten() // Remove option
                .cloned()
                .collect::<Vec<VertexPtr>>(),
        })
    }
}

// DAG construction: part 2
impl Bullshark {
    fn TryAdvanceRound(&mut self, outgoing: &mut OutgoingMessages<BullsharkMessage>) {
        if self.dag[self.round].len() >= self.QuorumSize() {
            self.round += 1;
            self.BroadcastVertex(self.round, outgoing);
        }
    }

    fn BroadcastVertex(&mut self, round: usize, outgoing: &mut OutgoingMessages<BullsharkMessage>) {
        let v = self.CreateVertex(round);
        self.TryAddToDAG(v.clone(), outgoing);
        outgoing.Broadcast(BullsharkMessage::Vertex(v));
    }

    fn TryAddToDAG(
        &mut self,
        v: VertexPtr,
        outgoing: &mut OutgoingMessages<BullsharkMessage>,
    ) -> bool {
        // Parents are not in the DAG yet
        if v.round - 1 > self.dag.CurrentMaxAllocatedRound() {
            return false;
        }

        self.dag.AddVertex(v.clone());

        if self.dag[v.round].len() >= self.QuorumSize() && v.round > self.round {
            self.round = v.round;
            self.BroadcastVertex(v.round, outgoing);
        }

        assert!(
            self.buffer.remove(&v),
            "Vertex should be in the buffer by that moment"
        );

        self.TryOrdering(v);

        return true;
    }
}

// Consensus logic
impl Bullshark {
    fn TryOrdering(&mut self, v: VertexPtr) {
        todo!()
    }
}
