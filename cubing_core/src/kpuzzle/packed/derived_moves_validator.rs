use std::collections::{HashMap, HashSet};

use crate::alg::{Alg, AlgNode, Move};

use super::{
    super::KPuzzleDefinition,
    kpuzzle::InvalidDefinitionError,
    lookup_move::{lookup_move, MoveLookupResultSource},
};

enum DerivedMoveVisitStatus {
    InProgress(()),
    Done(()),
}
// TODO: handle move definitions like x2.
pub(crate) struct DerivedMovesValidator<'a> {
    def: &'a KPuzzleDefinition,
    derived_move_visit_statuses: HashMap<Move, DerivedMoveVisitStatus>,
}

impl DerivedMovesValidator<'_> {
    pub fn check(def: &KPuzzleDefinition) -> Result<(), InvalidDefinitionError> {
        if let Some(derived_moves) = &def.derived_moves {
            let mut validator = DerivedMovesValidator {
                def,
                derived_move_visit_statuses: HashMap::default(),
            };
            for (derived_move, _) in derived_moves.iter() {
                validator.visit(derived_move)?
            }
        }
        Ok(())
    }

    fn visit(&mut self, key_move: &Move) -> Result<(), InvalidDefinitionError> {
        match self.derived_move_visit_statuses.get(key_move) {
            Some(DerivedMoveVisitStatus::InProgress(())) => {
                return Err(format!("Recursive derived move definition for: {}", key_move).into());
            }
            Some(DerivedMoveVisitStatus::Done(())) => return Ok(()),
            None => (),
        };
        self.derived_move_visit_statuses.insert(
            key_move.clone(), /* Avoid this clonse by using lifetimes? */
            DerivedMoveVisitStatus::InProgress(()),
        );
        let move_lookup_result = match lookup_move(self.def, key_move) {
            Some(move_lookup_result) => move_lookup_result,
            None => {
                return Err("Invalid move??".into());
            }
        };
        match move_lookup_result.source {
            MoveLookupResultSource::DirectlyDefined(_) => {}
            MoveLookupResultSource::DerivedFromAlg(alg) => {
                let descendant_move_keys = self.ancestor_move_keys_in_alg(alg)?;
                for descendant_move_key in descendant_move_keys {
                    self.visit(&descendant_move_key)?
                }
            }
        };
        // TODO: Would it help to save `descendant_move_keys` for something?
        self.derived_move_visit_statuses.insert(
            key_move.clone(), /* Avoid this clone by using lifetimes? */
            DerivedMoveVisitStatus::Done(()),
        );
        Ok(())
    }

    fn ancestor_move_keys_in_alg(&self, alg: &Alg) -> Result<HashSet<Move>, String> {
        let mut descendant_move_keys = HashSet::<Move>::default();
        for node in &alg.nodes {
            self.ancestor_move_keys_in_node(node, &mut descendant_move_keys)?
        }
        Ok(descendant_move_keys)
    }

    fn ancestor_move_keys_in_alg_recursive(
        &self,
        alg: &Alg,
        descendant_move_keys: &mut HashSet<Move>, // TODO: figure out how to avoid owning `Move`s?
    ) -> Result<(), String> {
        for node in &alg.nodes {
            self.ancestor_move_keys_in_node(node, descendant_move_keys)?
        }
        Ok(())
    }

    fn ancestor_move_keys_in_node<'a, 'b: 'a>(
        &self,
        node: &'a AlgNode,
        descendant_move_keys: &mut HashSet<Move>,
    ) -> Result<(), String> {
        match node {
            AlgNode::GroupingNode(grouping) => {
                self.ancestor_move_keys_in_alg_recursive(&grouping.alg, descendant_move_keys)?
            }
            AlgNode::CommutatorNode(commutator) => {
                self.ancestor_move_keys_in_alg_recursive(&commutator.a, descendant_move_keys)?;

                self.ancestor_move_keys_in_alg_recursive(&commutator.b, descendant_move_keys)?
            }
            AlgNode::ConjugateNode(conjugate) => {
                self.ancestor_move_keys_in_alg_recursive(&conjugate.a, descendant_move_keys)?;

                self.ancestor_move_keys_in_alg_recursive(&conjugate.b, descendant_move_keys)?
            }
            AlgNode::MoveNode(key_move) => {
                let move_lookup_result = match lookup_move(self.def, key_move) {
                    Some(move_lookup_result) => move_lookup_result,
                    None => {
                        return Err(format!(
                            "Invalid move used in a derived move definition: {}",
                            key_move
                        ))
                    }
                };
                descendant_move_keys.insert(move_lookup_result.key_move.clone());
                // TODO: figure out how to avoid the need to clone.
            }
            _ => (),
        };
        Ok(())
    }
}
