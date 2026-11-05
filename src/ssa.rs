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
//!         
//!
//!     ]
//!   }
//!   ]
//!
//! 
//! }
//!

use std::collections::HashMap;

/// A unique identifier for each value in an SSA program. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValueID(u32);

/// A unique identifier for each basic block in an SSA program.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BlockID(u32);

/// A unique identifier for each function in an SSA program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FunctionID(u32);

struct Module {
    functions: Vec<Function>,
}

/// Represents a function. Each function has a unique identifier.
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
struct Function {
    /// The unique identifier corresponding to the function defined in the given module.
    id: FunctionID,
    /// 
    params: Vec<ValueID>,
    /// The return type of the function. Type::Void if nothing is returned.
    return_type: Type,
    /// The basic block where execution begins.
    entry: BlockID,
    /// The list of basic blocks that are used to build the function.
    blocks: Vec<BasicBlock>,
    /// An association between every SSA value defined in the scope of the function and a type.
    value_types: HashMap<ValueID, Type>
}

/// Operands for instructions. For reference, the definition of an operand per
/// `en.wikipedia.org/wiki/Operand` is "the object of a mathematical operation".
/// Hence, Instructions like add act on operands (2 + 2)
///                                               |
///                                               ^
///                                             Operand
/// We do not store the type information of operands locally to save space. This
/// comes at the cost of requiring a lookup.
enum Operand {
    Value(ValueID),
    Constant {
        ty: Type,
        value: i64
    },

}

enum Type {
    Int,
    Pointer,
    Void

}

enum Instruction {
    Add {
        dest: ValueID,
        lhs: Operand,
        rhs: Operand
    },
    Sub {
        dest: ValueID,
        lhs: Operand,
        rhs: Operand
    },
    Mul {
        dest: ValueID,
        lhs: Operand,
        rhs: Operand
    },
    Phi {

    }
}

/// The minimal unit of structure in the SSA representation of a program. In our
/// model of SSA, we provide each block with an ID for lookup, a collection of
/// phi nodes to indicate possible control flow, a collection of instructions to
/// perform, and a terminator block determining to where control flows after
/// traversing the block.
struct BasicBlock {
    id: BlockID,
    phis: Vec<Instruction>,
    instructions: Vec<Instruction>,
    terminator: Terminator,
}

/// The terminator of a basic block functions to determine how control is passed
/// from the basic block to other parts of the program. This can be done in several
/// ways: (1) it can exit the current function, returning a value optionally;
/// (2) it can transfer control over to another basic block unconditionally akin to
/// a jump instruction; (3) it can conditionally branch and choose between two basic
/// blocks based on some condition.
enum Terminator {
    /// A return statement. Note that these exit the current _function_ in addition
    /// to transferring control out of the current basic block.
    Return(Option<Operand>),
    /// A branch statement. Unconditionally jumps to another basic block.
    Branch(BlockID),
    /// A conditional branch statement. 
    ConditionalBranch {
        /// The condition which will determine where control is transferred to.
        condition: Operand,
        /// The block jumped to if the condition is satisfied.
        then_block: BlockID,
        /// The block jumped to if the condition is not satisfied.
        else_block: BlockID
    }
}
