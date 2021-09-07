mod boxes;
mod derefs;
mod drop_trait;
mod ref_count;
mod limit_tracker;
mod rc_with_refcellt;
mod weak_references;

fn main() {
    boxes::boxes();
    derefs::derefs();
    drop_trait::run();
    ref_count::run();
    rc_with_refcellt::run();
    weak_references::run();
}