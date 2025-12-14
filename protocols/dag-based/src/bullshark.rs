// https://arxiv.org/pdf/2201.05677
// https://arxiv.org/pdf/2209.05633

use std::collections::{HashSet, VecDeque};

use simulator::*;

use crate::dag_utils::{RoundBasedDAG, SameVertex, Vertex, VertexPtr};

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
        match message {
            BullsharkMessage::Vertex(v) => {
                if v.strong_edges.len() < self.QuorumSize() || from != v.source {
                    return;
                }

                if !self.TryAddToDAG(v.clone(), outgoing) {
                    self.buffer.insert(v.clone());
                } else {
                    let vertices_in_the_buffer =
                        self.buffer.iter().cloned().collect::<Vec<VertexPtr>>();
                    vertices_in_the_buffer.into_iter().for_each(|v| {
                        self.TryAddToDAG(v, outgoing);
                    });
                }

                if v.round != self.round {
                    return;
                }

                let w = v.round.div_ceil(4);

                match v.round % 4 {
                    1 => {
                        if self.GetFirstPredefinedLeader(w) == v.source {
                            self.TryAdvanceRound(outgoing);
                        }
                    }
                    3 => {
                        if self.GetSecondPredefinedLeader(w) == v.source {
                            self.TryAdvanceRound(outgoing);
                        }
                    }
                    0 => {
                        todo!()
                    }
                    4 => {
                        todo!()
                    }
                    _ => unreachable!(),
                }
            }
        }
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

    fn GetFirstPredefinedLeader(&self, w: usize) -> ProcessId {
        let round = 4 * w - 3;
        return self.GetLeaderId(round);
    }

    fn GetSecondPredefinedLeader(&self, w: usize) -> ProcessId {
        let round = 4 * w - 1;
        return self.GetLeaderId(round);
    }

    fn GetLeaderId(&self, round: usize) -> ProcessId {
        return round % self.proc_num;
    }
}

// DAG construction: part 2
impl Bullshark {
    fn TryAdvanceRound(&mut self, outgoing: &mut OutgoingMessages<BullsharkMessage>) {
        if self.dag[self.round].iter().flatten().count() >= self.QuorumSize() {
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
        // Strong edges are not in the DAG yet
        if v.round - 1 > self.dag.CurrentMaxAllocatedRound() {
            return false;
        }

        let all_strong_edges_in_the_dag =
            v.strong_edges
                .iter()
                .all(|edge| match self.dag[edge.round][edge.source] {
                    None => false,
                    Some(ref vertex) => SameVertex(&edge, vertex),
                });

        if !all_strong_edges_in_the_dag {
            return false;
        }

        self.dag.AddVertex(v.clone());

        if self.dag[self.round].iter().flatten().count() >= self.QuorumSize()
            && v.round > self.round
        {
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
