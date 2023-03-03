use std::rc::Rc;

use super::{KPuzzleDefinition, KTransformation};

pub struct KPuzzle {
    pub definition: Rc<KPuzzleDefinition>,
}

impl KPuzzle {
    pub fn transformation_from_move(
        &self,
        r#move: crate::alg::Move,
    ) -> Result<KTransformation, String> {
        let s = r#move.to_string();
        let a = self.definition.moves.get(&s).ok_or("Unknown move name.")?;
        Ok(KTransformation {
            transformation_data: a.clone(),
        })
    }
}
