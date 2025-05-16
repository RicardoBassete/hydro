use crate::parser::NodeExit;

pub struct Generator {
    root: NodeExit,
}

impl Generator {
    pub fn new(root: NodeExit) -> Generator {
        Generator { root: root }
    }

    pub fn generate(&self) -> String {
        let mut result = String::from("global _start\n_start:\n");
        result.push_str("    mov rax, 60\n");
        result.push_str(&format!(
            "    mov rdi, {}\n",
            self.root.expr.int_lit.value.clone().unwrap()
        ));
        result.push_str("    syscall");

        result
    }
}
