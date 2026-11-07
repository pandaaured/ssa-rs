//! This test module tests straight-line arithmetic with no control flow.

#[cfg(test)]
pub mod tests {
    use ssa_rs::ssa::{BlockID, CFGBasicBlock, Function, FunctionID, 
        Instruction, Operand, Terminator, Type, VarID};

    use std::collections::HashMap;

    #[test]
    /// This CFG represents the following program.
    ///
    /// ----- BLOCK -----
    /// fn (a : Int) : Int =
    ///   let x = a + 1
    ///   let x = x + 1
    ///   x
    /// --- END BLOCK ---
    fn cfg_to_ssa_adder() {
        let a = VarID(0);
        let x = VarID(1);
        let entry = BlockID(0);

        let added_to_value= Operand::cfg_new_int(1);
        let added_to_value_two= Operand::cfg_new_int(1);

        let data = Function {
            id: FunctionID(0),
            params: vec![a],
            return_type: Type::Int,
            entry,
            blocks: vec![CFGBasicBlock {
                id: entry,
                instructions: vec![
                    Instruction::Add {
                        dest: x,
                        lhs: Operand::Value(a),
                        rhs: added_to_value,
                    },
                    Instruction::Add {
                        dest: x,
                        lhs: Operand::Value(x),
                        rhs: added_to_value_two                    },
                ],
                terminator: Terminator::Return(Some(Operand::Value(x))),
            }],
            value_types: HashMap::from([
                (a, Type::Int),
                (x, Type::Int),
            ]),
        };
    }

    #[test]
    /// This CFG represents the following program.
    ///
    /// ----- BLOCK -----
    /// fn (() : Void) : Int =
    ///   if 2 = 3
    ///   0 <> 1
    /// --- END BLOCK ---
    ///
    /// We test for equality of two constants and return one constant or the
    /// other depending on the value. Obviously, this should only ever return
    /// one of these paths. Writing this program now so it can be optimized
    /// later.
    fn cfg_to_ssa_cond() {
        // Basic blocks composing our program.
        let entry = BlockID(0);
        let exitfalse = BlockID(1);
        let exittrue = BlockID(2);

        // Variables compsing our program.
        let a = VarID(0);

        // Constants composing our program.
        let const_one = Operand::cfg_new_int(2);
        let const_two = Operand::cfg_new_int(3);

        // Instructions composing our program.
        let eq_check = Instruction::equal(a, const_one, const_two);

        let data = Function {
            id: FunctionID(0),
            params: vec![],
            return_type: Type::Int,
            entry,
            blocks: vec![CFGBasicBlock {
                id: entry,
                instructions: vec![eq_check],
                terminator: Terminator::ConditionalBranch { 
                    condition: Operand::Value(a), then_block: exittrue, else_block: exitfalse
                }
            },
            CFGBasicBlock::new_terminator_return_const(exittrue, 1),
            CFGBasicBlock::new_terminator_return_const(exitfalse, 1),
            ],
            value_types: HashMap::from([
                (a, Type::Bool),
            ])
        };
    }



}
