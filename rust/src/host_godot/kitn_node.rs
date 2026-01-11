use godot::prelude::*;
use crate::vm::runtime::PocketPy;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct KitnNode {
    vm: Option<PocketPy>,
    base: Base<Node>,
}

#[godot_api]
impl INode for KitnNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            vm: None,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("KitnNode ready!");

        let mut vm = PocketPy::new();
        vm.bind_print();

        godot_print!("Executing Python script...");
        vm.exec("print('Hello from PocketPy on Godot!')");

        self.vm = Some(vm);
    }
}
