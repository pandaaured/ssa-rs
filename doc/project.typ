#set text(font: "Palatino")

= Status
== November 12, 2026
After experimentation, we have settled on a design of the key data structures
for this project. We take inspiration from the hierarchy of program components
of LLVM: the highest-level structures are *modules*. Modules are akin to 
translation units in C and C++. Modules contain *functions* which match the 
behavior one would expect from functions.

A key unit in static single-assignment form is the *basic block*. The basic
block is the minimal unit with respect to control flow through which we
execute instructions. Basic blocks consist of zero or more instructions
followed by a *terminator*: a special instruction that transfers control
flow from the current location to another. This can occur in several ways:
we can return a value, we can return void (a function that returns no value),
we can unconditionally jump to another basic block, or we can conditionally
jump to a basic block (where we have the equivalent of an `if condition
then jump BlockID(1) else jump BlockID(2)`)

_Modules_ contain the following data: a vector of functions.
_Functions_ contain the following data: an *identifier*: a unique integer
that distinguishes a function from all others in the program; *parameters*:
inputs to the function that are represented with unique identifiers; a
*return type* which determines the structure of the values returned from the
function; a *basic block* which determines the location where control flow
begins in the function (stored as a unique identifier); a *list of basic 
blocks* which contain all of the instructions which determine the function's
behavior; a *value types map* which associates every value identifier with
a type.
