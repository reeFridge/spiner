use slot::Slot;

pub trait Vertex {
    fn world_vertices_len(&self) -> usize;
    fn compute_world_vertices(
        &self,
        slot: &Slot,
        start: i32,
        count: i32,
        vertices: &mut Vec<f32>,
        offset: i32,
        stride: i32,
    );
}
