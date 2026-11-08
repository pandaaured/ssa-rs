use crate::ssa::Operand;

macro_rules! def_instructions {
    (
        $(
            $method:ident => $variant:ident(
                $($operand:ident),+ $(,)?
            )
        ),+ $(,)?
    ) => {
        pub enum Instruction<Id> {
            $(
                $variant {
                    dest: Id,
                    $(
                        $operand: Operand<Id>,
                    )+
                },
            )+
        }

        impl<Id> Instruction<Id> {
            $(
                pub fn $method(
                    dest: Id,
                    $(
                        $operand: Operand<Id>,
                    )+
                ) -> Self {
                    Self::$variant {
                        dest,
                        $($operand,)+
                    }
                }
            )+
        }
    };
}

// We define the following instructions to perform arithmetic and logical
// operations. There is some correspondence between these operations and
// those in the LLVM Language Reference.
//
// We currently have the following loose types of instructions:
// Binary Operations, comprising
// add, sub, mul, div
def_instructions! {
    add => Add(lhs, rhs),
    sub => Sub(lhs, rhs),
    mul => Mul(lhs, rhs),
    div => Div(lhs, rhs),
    eq => Eq(lhs, rhs),
    lt => Lt(lhs, rhs),
    gt => Gt(lhs, rhs),
    leq => Leq(lhs, rhs),
    geq => Geq(lhs, rhs),

    neg => Neg(op),
    not => Not(op)
}


