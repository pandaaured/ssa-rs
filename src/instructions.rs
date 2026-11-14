use crate::ssa::Operand;
use crate::types::Type;
use crate::ssa::ValueID;

pub struct WrapFlags {
    pub nuw: bool,
    pub nsw: bool
}

pub struct Unreachable;

pub enum Ret {
    Void,
    Value(Operand)
}

macro_rules! oper {
    ( $ ($struct_name:ident => $fn_name:ident),* ) => {
        $( pub struct $struct_name {
            pub lhs: Operand,
            pub rhs: Operand,
            pub dest: ValueID,
            pub ty: Type,
        }
        
        impl $struct_name {
            pub fn $fn_name(lhs: Operand, rhs: Operand, dest: ValueID, ty: Type) -> $struct_name {
                $struct_name { lhs, rhs, dest, ty }
            }
        })*
    };
}

macro_rules! oper_wrap_flag {
    ( $ ($struct_name:ident => $fn_name:ident),* ) => {
        $( pub struct $struct_name {
            pub lhs: Operand,
            pub rhs: Operand,
            pub dest: ValueID,
            pub ty: Type,
            pub flags: WrapFlags,
        }
        
        impl $struct_name {
            pub fn $fn_name(lhs: Operand, rhs: Operand, dest: ValueID, ty: Type, flags: WrapFlags) -> $struct_name {
                $struct_name { lhs, rhs, dest, ty, flags }
            }
        })*
    };
}

macro_rules! oper_exact {
    ( $ ($struct_name:ident => $fn_name:ident),* ) => {
        $( pub struct $struct_name {
            pub lhs: Operand,
            pub rhs: Operand,
            pub dest: ValueID,
            pub ty: Type,
            pub exact: bool,
        }
        
        impl $struct_name {
            pub fn $fn_name(lhs: Operand, rhs: Operand, dest: ValueID, ty: Type, exact: bool) -> $struct_name {
                $struct_name { lhs, rhs, dest, ty, exact }
            }
        })*
    };
}

macro_rules! oper_disjoint {
    ( $ ($struct_name:ident => $fn_name:ident),* ) => {
        $( pub struct $struct_name {
            pub lhs: Operand,
            pub rhs: Operand,
            pub dest: ValueID,
            pub ty: Type,
            pub disjoint: bool,
        }
        
        impl $struct_name {
            pub fn $fn_name(lhs: Operand, rhs: Operand, dest: ValueID, ty: Type, disjoint: bool) -> $struct_name {
                $struct_name { lhs, rhs, dest, ty, disjoint }
            }
        })*
    };
}

pub enum Cond {
    Eq,
    Ne,
    UGT,
    ULT,
    ULE,
    SGT,
    SGE,
    SLT,
    SLE
}

pub struct ICmp {
    pub lhs: Operand,
    pub rhs: Operand,
    pub dest: ValueID,
    pub ty: Type,
    pub cond: Cond,
    pub samesign: bool
}

impl ICmp {
    pub fn icmp(lhs: Operand, rhs: Operand, dest: ValueID, ty: Type, cond: Cond, samesign: bool) -> ICmp {
       ICmp { lhs, rhs, dest, ty, cond, samesign }  
    }
}

oper!(Urem => urem, Srem => srem, And => and);
oper_wrap_flag!(Add => add, Sub => sub, Mul => mul, Div => div, Shl => shl);
oper_exact!(Udiv => udiv, Sdiv => sdiv, Lshr => lshr, Ashr => ashr);
oper_disjoint!(Or => or);

pub enum Instruction {
    Ret(Ret),
    Unreachable(Unreachable),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Udiv(Udiv),
    Sdiv(Sdiv),
    Urem(Urem),
    Srem(Srem),
    Shl(Shl),
    Lshr(Lshr),
    Ashr(Ashr),
    And(And),
    ICmp(ICmp)
}

