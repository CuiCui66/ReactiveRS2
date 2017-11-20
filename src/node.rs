use engine::Runtime;

pub trait Node<'a> {
    fn call(&mut self, runtime: &mut Runtime);
}
