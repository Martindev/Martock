//! arbiter defends the world from invalid modifications.

use commit;
use committer;
use world;

pub struct Arbiter;

/// Arbiter decides whether to apply commits to the world.
impl Arbiter {
    pub fn new() -> Self {
        Arbiter {}
    }

    pub fn arbitrate(&self, cls: Vec<Box<committer::CL>>, w: &mut world::World) {
        for cl in cls {
            for commit in cl {
                if self.approve(&commit) {
                    w.set_block(commit.x, commit.y, commit.next_state);
                }
            }
        }
    }

    fn approve(&self, _: &commit::Commit) -> bool {
        true
    }
}
