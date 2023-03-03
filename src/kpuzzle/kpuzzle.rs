use std::rc::Rc;

use super::{KPuzzleDefinition, KTransformation};

pub struct KPuzzle {
    pub definition: Rc<KPuzzleDefinition>,
}

// TODO: Get rid of this in favor of purely `KTransformation` and `KState`?
impl KPuzzle {
    pub fn transformation_from_move(
        &self, // TODO: Any issues with not using `&self`?
        r#move: crate::alg::Move,
    ) -> Result<KTransformation, String> {
        let s = r#move.to_string();
        let a = self.definition.moves.get(&s).ok_or("Unknown move name.")?;
        Ok(KTransformation {
            definition: self.definition.clone(),
            transformation_data: a.clone(),
        })
    }
}
