//! This module defines the SSA language we are implementing.
//!
//! Suppose we have the following LLVM IR and would like to represent it as a
//! collection of data structures such that we can parse it, apply transformations
//! onto it, and optimize it.
//!
//! ```ll
//! define i32 @add(i32 x, i32 y) {
//!   %add = add i32 %x, %y
//!   ret i32 %add
//! }
//! ```
//! This function does not have any control flow constructs, so it suffices to
//! encode the above as a basic block. Our representation of basic blocks includes
//! a terminator at the end.
//!
//! We describe below how we encode this function in our implementation of SSA.
//! At the top level, we have the Module which is analogous to the "Translation Unit".
//! This contains all of the functions defined in our program.
//!
//!         Module --v
//! -----------------------------
//! |         Function          |
//! |         Function          |
//! |         Function          |
//! -----------------------------
//! 
//! We would represent the above function like so:
//!
//! Function {
//!   id: 1, // function ID
//!   params: [1, 2], // value IDs
//!   return_type: Type::Int,
//!   blocks: [ BasicBlock {
//!     id: 1, // basic block ID
//!     phis: [],
//!     instructions: [
//!       TODO: Finish this 
//!
//!     ]
//!   }
//!   ]
//!
//! 
//! }
//!

use std::collections::{HashMap, HashSet};

/// A unique identifier for each variable in a CFG representation of a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarID(pub u32);

/// A unique identifier for each value in an SSA program. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(pub u32);

/// A unique identifier for each basic block in an SSA program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub u32);

/// A unique identifier for each function in an SSA program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionID(pub u32);

/// The coarsest granularity for program representation currently supported by
/// the AST. A module simply contains the functions that a program needs to run.
pub struct Module<Func> {
    pub functions: Vec<Func>,
}

/// Encodes a function in the Control Flow Graph representation.
pub type CFGFunction = Function<VarID, CFGBasicBlock>;

/// Encodes a function in the Static Single-Assignment representation.
pub type SSAFunction = Function<ValueID, BasicBlock>;

/// Represents a function. Each function has a unique identifier. We define the function
/// to be generic over the type of the ID and the Block type, which we pass the types
/// CFGFunction and SSAFunction into Module to obtain.
///
/// Consider again the example
/// ```ll
/// define i32 @add(i32 x, i32 y) {
///   %add = add i32 %x, %y
///   ret i32 %add
/// }
///
/// The return type is i32 in LLVM IR, so we communicate this by using Type::Int
/// as the return_type field's value.
pub struct Function<Id, Block> {
    /// The unique identifier corresponding to the function defined in the given module.
    pub id: FunctionID,
    pub params: Vec<Id>,
    /// The return type of the function. Type::Void if nothing is returned.
    pub return_type: Type,
    /// The basic block where execution begins.
    pub entry: BlockID,
    /// The list of basic blocks that are used to build the function.
    pub blocks: Vec<Block>,
    /// An association between every value or variable defined in the scope of the function and a type.
    pub value_types: HashMap<Id, Type>
}

impl CFGFunction {
    /// Given a list of blocks in a function, return a map of which blocks are
    /// predecessors of other blocks for generation of phi functions.
    pub fn predecessors(&self) -> HashMap<BlockID, HashSet<BlockID>> {
        let mut predecessors: HashMap<BlockID, HashSet<BlockID>> = HashMap::new();
        for block in &self.blocks {
            match &block.terminator {
                Terminator::Return(_) => {}
                Terminator::Branch(target) => {
                    predecessors.entry(*target).or_default().insert(block.id);
                }
                Terminator::ConditionalBranch { then_block, else_block, .. } =>
                {
                    predecessors.entry(*then_block).or_default().insert(block.id);
                    predecessors.entry(*else_block).or_default().insert(block.id);
                }
            }
        }

        predecessors
    }
}

/// Operands for instructions. For reference, the definition of an operand per
/// `en.wikipedia.org/wiki/Operand` is "the object of a mathematical operation".
/// Hence, Instructions like add act on operands (2 + 2)
///                                               |
///                                               ^
///                                             Operand
/// We do not store the type information of operands locally to save space. This
/// comes at the cost of requiring a lookup.
pub enum Operand<Id> {
    Value(Id),
    Constant {
        ty: Type,
        value: i64
    },
}

impl Operand<VarID> {
    pub fn cfg_new_int(value: i64) -> Self {
        Self::Constant { ty: Type::Int, value }
    }
}

impl Operand<ValueID> {
    pub fn ssa_new_int(value: i64) -> Self {
        Self::Constant { ty: Type::Int, value }
    }
}

pub enum Type {
    Int,
    Pointer,
    Void,
    Bool
}


/// We make instructions generic over the CFG and SSA representations of our
/// programs. We have the following instructions in our SSA language:
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


pub type CFGInstruction = Instruction<VarID>;
pub type SSAInstruction = Instruction<ValueID>;

/// The minimal unit of structure in the SSA representation of a program. In our
/// model of SSA, we provide each block with an ID for lookup, a collection of
/// phi nodes to indicate possible control flow, a collection of instructions to
/// perform, and a terminator block determining to where control flows after
/// traversing the block.
pub struct BasicBlock {
    pub id: BlockID,
    pub phis: Vec<SSAInstruction>,
    pub instructions: Vec<SSAInstruction>,
    pub terminator: Terminator<ValueID>,
}


/// The minimal unit of structure in the CFG representation of a program. It is
/// structurally almost identical to the SSA representation modulo the lack of
/// a set of phi nodes. We also refer to variable assignments as opposed to value
/// assignments.
pub struct CFGBasicBlock {
    pub id: BlockID,
    pub instructions: Vec<CFGInstruction>,
    pub terminator: Terminator<VarID>,
}

impl CFGBasicBlock {
    /// Given a block ID and a value, set your block to contain only a terminator
    /// with a return instruction of a constant type.
    pub fn new_terminator_return_const(id: BlockID, value: i64) -> Self {
        Self { id, 
            instructions: vec![], 
            terminator: Terminator::Return(Some(Operand::Constant { ty: Type::Int, value }))
        }
    }
}


/// The terminator of a basic block functions to determine how control is passed
/// from the basic block to other parts of the program. This can be done in several
/// ways: (1) it can exit the current function, returning a value optionally;
/// (2) it can transfer control over to another basic block unconditionally akin to
/// a jump instruction; (3) it can conditionally branch and choose between two basic
/// blocks based on some condition. Note that we can use terminators for both
/// `BasicBlock`s and `CFGBasicBlocks` which lack `phis`.
///  
pub enum Terminator<Id> {
    /// A return statement. Note that these exit the current _function_ in addition
    /// to transferring control out of the current basic block.
    Return(Option<Operand<Id>>),
    /// A branch statement. Unconditionally jumps to another basic block.
    Branch(BlockID),
    /// A conditional branch statement. 
    ConditionalBranch {
        /// The condition which will determine where control is transferred to.
        condition: Operand<Id>,
        /// The block jumped to if the condition is satisfied.
        then_block: BlockID,
        /// The block jumped to if the condition is not satisfied.
        else_block: BlockID
    }
}

