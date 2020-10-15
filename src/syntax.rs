use alloc::boxed::Box;
use alloc::fmt;
use alloc::vec::Vec;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

fn fmt_list<T: fmt::Display>(
    f: &mut fmt::Formatter,
    sep: &'static str,
    args: &[T],
) -> fmt::Result {
    if args.is_empty() {
        return Ok(());
    }

    let mut args = args.iter();
    write!(f, "{}", args.next().unwrap())?;
    for arg in args {
        write!(f, "{}{}", sep, arg)?;
    }
    Ok(())
}

macro_rules! impl_unit_display {
    ($Type:ident) => {
        impl fmt::Display for $Type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_unit_anon_display {
    ($Type:ident) => {
        impl fmt::Display for $Type<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_enum_anon_display {
    ($Type:ident, $($Tag:ident),*) => {
        impl fmt::Display for $Type<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        $Type::$Tag(data) => write!(f, "{}", data),
                    )*
                }
            }
        }
    };
}

/// `integer`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Integer<'a>(pub &'a str);
impl_unit_anon_display!(Integer);

/// `rational`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Rational<'a>(pub &'a str);
impl_unit_anon_display!(Rational);

/// `real`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Real<'a>(pub &'a str);
impl_unit_anon_display!(Real);

/// `lower_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LowerWord<'a>(pub &'a str);
impl_unit_anon_display!(LowerWord);

/// `upper_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UpperWord<'a>(pub &'a str);
impl_unit_anon_display!(UpperWord);

/// `dollar_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DollarWord<'a>(pub LowerWord<'a>);

impl<'a> fmt::Display for DollarWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

/// `dollar_dollar_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DollarDollarWord<'a>(pub LowerWord<'a>);

impl<'a> fmt::Display for DollarDollarWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$${}", self.0)
    }
}

/// `single_quoted`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SingleQuoted<'a>(pub &'a str);

impl<'a> fmt::Display for SingleQuoted<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.0)
    }
}

/// `distinct_object`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DistinctObject<'a>(pub &'a str);

impl<'a> fmt::Display for DistinctObject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

/// `atomic_system_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AtomicSystemWord<'a>(pub DollarDollarWord<'a>);
impl_unit_anon_display!(AtomicSystemWord);

/// `system_functor`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SystemFunctor<'a>(pub AtomicSystemWord<'a>);
impl_unit_anon_display!(SystemFunctor);

/// `system_constant`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SystemConstant<'a>(pub SystemFunctor<'a>);
impl_unit_anon_display!(SystemConstant);

/// `number`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Number<'a> {
    Integer(Integer<'a>),
    Rational(Rational<'a>),
    Real(Real<'a>),
}
impl_enum_anon_display!(Number, Integer, Rational, Real);

/// `atomic_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AtomicWord<'a> {
    Lower(LowerWord<'a>),
    SingleQuoted(SingleQuoted<'a>),
}
impl_enum_anon_display!(AtomicWord, Lower, SingleQuoted);

/// `name`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Name<'a> {
    AtomicWord(AtomicWord<'a>),
    Integer(Integer<'a>),
}
impl_enum_anon_display!(Name, AtomicWord, Integer);

/// `variable`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Variable<'a>(pub UpperWord<'a>);
impl_unit_anon_display!(Variable);

/// `functor`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Functor<'a>(pub AtomicWord<'a>);
impl_unit_anon_display!(Functor);

/// `constant`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Constant<'a>(pub Functor<'a>);
impl_unit_anon_display!(Constant);

/// `fof_arguments`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofArguments<'a>(pub Vec<FofTerm<'a>>);

impl<'a> fmt::Display for FofArguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, "(")?;
        fmt_list(f, ",", &self.0)?;
        write!(f, ")")
    }
}

/// `fof_system_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofSystemTerm<'a> {
    /// `system_constant`
    Constant(SystemConstant<'a>),
    /// `system_functor`, `fof_arguments`
    Function(SystemFunctor<'a>, FofArguments<'a>),
}

impl<'a> fmt::Display for FofSystemTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FofSystemTerm::*;
        match self {
            Constant(c) => write!(f, "{}", c),
            Function(name, args) => write!(f, "{}{}", name, args),
        }
    }
}

/// `fof_plain_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofPlainTerm<'a> {
    /// `constant`
    Constant(Constant<'a>),
    /// `functor`, `fof_arguments`
    Function(Functor<'a>, FofArguments<'a>),
}

impl<'a> fmt::Display for FofPlainTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FofPlainTerm::*;
        match self {
            Constant(c) => write!(f, "{}", c),
            Function(name, args) => write!(f, "{}{}", name, args),
        }
    }
}

/// `atomic_defined_word`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AtomicDefinedWord<'a>(pub DollarWord<'a>);
impl_unit_anon_display!(AtomicDefinedWord);

/// `defined_functor`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DefinedFunctor<'a>(pub AtomicDefinedWord<'a>);
impl_unit_anon_display!(DefinedFunctor);

/// `defined_constant`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DefinedConstant<'a>(pub DefinedFunctor<'a>);
impl_unit_anon_display!(DefinedConstant);

/// `fof_defined_plain_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofDefinedPlainTerm<'a>(pub DefinedConstant<'a>);
impl_unit_anon_display!(FofDefinedPlainTerm);

/// `fof_defined_atomic_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofDefinedAtomicTerm<'a>(pub FofDefinedPlainTerm<'a>);
impl_unit_anon_display!(FofDefinedAtomicTerm);

/// `defined_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum DefinedTerm<'a> {
    Number(Number<'a>),
    Distinct(DistinctObject<'a>),
}
impl_enum_anon_display!(DefinedTerm, Number, Distinct);

/// `fof_defined_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofDefinedTerm<'a> {
    Defined(DefinedTerm<'a>),
    Atomic(FofDefinedAtomicTerm<'a>),
}
impl_enum_anon_display!(FofDefinedTerm, Defined, Atomic);

/// `fof_function_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofFunctionTerm<'a> {
    /// `fof_plain_term`
    Plain(FofPlainTerm<'a>),
    /// `fof_defined_term`
    Defined(FofDefinedTerm<'a>),
}
impl_enum_anon_display!(FofFunctionTerm, Plain, Defined);

/// `fof_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofTerm<'a> {
    /// `fof_function_term`,
    Function(FofFunctionTerm<'a>),
    /// `variable`
    Variable(Variable<'a>),
}
impl_enum_anon_display!(FofTerm, Function, Variable);

/// `unary_connective`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UnaryConnective;

impl fmt::Display for UnaryConnective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "~")
    }
}

/// `infix_equality`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InfixEquality;

impl fmt::Display for InfixEquality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=")
    }
}

/// `infix_inequality`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InfixInequality;

impl fmt::Display for InfixInequality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!=")
    }
}

/// `nonassoc_connective`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum NonassocConnective {
    /// `=>`
    LRImplies,
    /// `<=`
    RLImplies,
    /// `<=>`
    Equivalent,
    /// `<~>`
    NotEquivalent,
    /// `~|`
    NotOr,
    /// `~&`
    NotAnd,
}

impl fmt::Display for NonassocConnective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NonassocConnective::*;
        match self {
            LRImplies => write!(f, "=>"),
            RLImplies => write!(f, "<="),
            Equivalent => write!(f, "<=>"),
            NotEquivalent => write!(f, "<~>"),
            NotOr => write!(f, "~|"),
            NotAnd => write!(f, "~&"),
        }
    }
}

/// `assoc_connective`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AssocConnective {
    /// `&`
    And,
    /// `|`
    Or,
}

impl fmt::Display for AssocConnective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AssocConnective::*;
        match self {
            And => write!(f, "&"),
            Or => write!(f, "|"),
        }
    }
}

/// `fof_quantifier`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofQuantifier {
    /// `!`
    Forall,
    /// `?`
    Exists,
}

impl fmt::Display for FofQuantifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FofQuantifier::*;
        match self {
            Forall => write!(f, "!"),
            Exists => write!(f, "?"),
        }
    }
}

/// `fof_system_atomic_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofSystemAtomicFormula<'a>(pub FofSystemTerm<'a>);
impl_unit_anon_display!(FofSystemAtomicFormula);

/// `fof_plain_atomic_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofPlainAtomicFormula<'a>(pub FofPlainTerm<'a>);
impl_unit_anon_display!(FofPlainAtomicFormula);

/// `defined_infix_pred`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DefinedInfixPred(pub InfixEquality);
impl_unit_display!(DefinedInfixPred);

/// `fof_defined_infix_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofDefinedInfixFormula<'a> {
    pub left: Box<FofTerm<'a>>,
    pub op: DefinedInfixPred,
    pub right: Box<FofTerm<'a>>,
}

impl<'a> fmt::Display for FofDefinedInfixFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.op, self.right)
    }
}

/// `fof_defined_plain_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofDefinedPlainFormula<'a>(pub FofDefinedPlainTerm<'a>);
impl_unit_anon_display!(FofDefinedPlainFormula);

/// `fof_defined_atomic_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofDefinedAtomicFormula<'a> {
    Plain(FofDefinedPlainFormula<'a>),
    Infix(FofDefinedInfixFormula<'a>),
}
impl_enum_anon_display!(FofDefinedAtomicFormula, Plain, Infix);

/// `fof_atomic_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofAtomicFormula<'a> {
    Plain(FofPlainAtomicFormula<'a>),
    Defined(FofDefinedAtomicFormula<'a>),
    System(FofSystemAtomicFormula<'a>),
}
impl_enum_anon_display!(FofAtomicFormula, Plain, Defined, System);

/// `fof_variable_list`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofVariableList<'a>(pub Vec<Variable<'a>>);

impl<'a> fmt::Display for FofVariableList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, ",", &self.0)
    }
}

/// `fof_quantified_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofQuantifiedFormula<'a> {
    pub quantifier: FofQuantifier,
    pub bound: FofVariableList<'a>,
    pub formula: Box<FofUnitFormula<'a>>,
}

impl<'a> fmt::Display for FofQuantifiedFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]:{}", self.quantifier, self.bound, self.formula)
    }
}

/// `fof_infix_unary`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofInfixUnary<'a> {
    pub left: Box<FofTerm<'a>>,
    pub op: InfixInequality,
    pub right: Box<FofTerm<'a>>,
}

impl<'a> fmt::Display for FofInfixUnary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.op, self.right)
    }
}

/// `fof_unary_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofUnaryFormula<'a> {
    Unary(UnaryConnective, Box<FofUnitFormula<'a>>),
    InfixUnary(FofInfixUnary<'a>),
}

impl<'a> fmt::Display for FofUnaryFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FofUnaryFormula::*;
        match self {
            Unary(connective, u) => write!(f, "{}{}", connective, u),
            InfixUnary(i) => write!(f, "{}", i),
        }
    }
}

/// `fof_unitary_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofUnitaryFormula<'a> {
    Quantified(FofQuantifiedFormula<'a>),
    Atomic(FofAtomicFormula<'a>),
    Parenthesised(Box<FofLogicFormula<'a>>),
}

impl<'a> fmt::Display for FofUnitaryFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FofUnitaryFormula::*;
        match self {
            Quantified(q) => write!(f, "{}", q),
            Atomic(a) => write!(f, "{}", a),
            Parenthesised(p) => write!(f, "({})", p),
        }
    }
}

/// `fof_unit_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofUnitFormula<'a> {
    Unitary(FofUnitaryFormula<'a>),
    Unary(FofUnaryFormula<'a>),
}
impl_enum_anon_display!(FofUnitFormula, Unitary, Unary);

/// `fof_or_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofOrFormula<'a>(pub Vec<FofUnitFormula<'a>>);

impl<'a> fmt::Display for FofOrFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, "|", &self.0)
    }
}

/// `fof_and_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofAndFormula<'a>(pub Vec<FofUnitFormula<'a>>);

impl<'a> fmt::Display for FofAndFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, "&", &self.0)
    }
}

/// `fof_binary_assoc`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofBinaryAssoc<'a> {
    Or(FofOrFormula<'a>),
    And(FofAndFormula<'a>),
}
impl_enum_anon_display!(FofBinaryAssoc, Or, And);

/// `fof_binary_nonassoc`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofBinaryNonassoc<'a> {
    pub left: Box<FofUnitFormula<'a>>,
    pub op: NonassocConnective,
    pub right: Box<FofUnitFormula<'a>>,
}

impl<'a> fmt::Display for FofBinaryNonassoc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.op, self.right)
    }
}

/// `fof_binary_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofBinaryFormula<'a> {
    Nonassoc(FofBinaryNonassoc<'a>),
    Assoc(FofBinaryAssoc<'a>),
}
impl_enum_anon_display!(FofBinaryFormula, Nonassoc, Assoc);

/// `fof_logic_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FofLogicFormula<'a> {
    Binary(FofBinaryFormula<'a>),
    Unary(FofUnaryFormula<'a>),
    Unitary(FofUnitaryFormula<'a>),
}
impl_enum_anon_display!(FofLogicFormula, Binary, Unary, Unitary);

/// `fof_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofFormula<'a>(pub FofLogicFormula<'a>);
impl_unit_anon_display!(FofFormula);

/// `literal`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Literal<'a> {
    Atomic(FofAtomicFormula<'a>),
    NegatedAtomic(FofAtomicFormula<'a>),
    Infix(FofInfixUnary<'a>),
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Literal::*;
        match self {
            Atomic(a) => write!(f, "{}", a),
            NegatedAtomic(n) => write!(f, "~{}", n),
            Infix(i) => write!(f, "{}", i),
        }
    }
}

/// `disjunction`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Disjunction<'a>(pub Vec<Literal<'a>>);

impl<'a> fmt::Display for Disjunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, "|", &self.0)
    }
}

/// `cnf_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CnfFormula<'a> {
    Disjunction(Disjunction<'a>),
    Parenthesised(Disjunction<'a>),
}

impl<'a> fmt::Display for CnfFormula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CnfFormula::*;
        match self {
            Disjunction(d) => write!(f, "{}", d),
            Parenthesised(d) => write!(f, "({})", d),
        }
    }
}

/// `formula_role`
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FormulaRole {
    Axiom,
    Hypothesis,
    Definition,
    Assumption,
    Lemma,
    Theorem,
    Corollary,
    Conjecture,
    NegatedConjecture,
    Plain,
    Type,
    FiDomain,
    FiFunctors,
    FiPredicates,
    Unknown,
}

impl fmt::Display for FormulaRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FormulaRole::*;
        match self {
            Axiom => write!(f, "axiom"),
            Hypothesis => write!(f, "hypothesis"),
            Definition => write!(f, "definition"),
            Assumption => write!(f, "assumption"),
            Lemma => write!(f, "lemma"),
            Theorem => write!(f, "theorem"),
            Corollary => write!(f, "corollary"),
            Conjecture => write!(f, "conjecture"),
            NegatedConjecture => write!(f, "negated_conjecture"),
            Plain => write!(f, "plain"),
            Type => write!(f, "type"),
            FiDomain => write!(f, "fi_domain"),
            FiFunctors => write!(f, "fi_functors"),
            FiPredicates => write!(f, "fi_predicates"),
            Unknown => write!(f, "unknown"),
        }
    }
}

/// `formula_data`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FormulaData<'a> {
    Fof(Box<FofFormula<'a>>),
    Cnf(Box<CnfFormula<'a>>),
    Fot(Box<FofTerm<'a>>),
}

impl<'a> fmt::Display for FormulaData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FormulaData::*;
        match self {
            Fof(fof) => write!(f, "$fof({})", fof),
            Cnf(cnf) => write!(f, "$cnf({})", cnf),
            Fot(cnf) => write!(f, "$fot({})", cnf),
        }
    }
}

/// `general_function`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GeneralFunction<'a> {
    pub word: AtomicWord<'a>,
    pub terms: GeneralTerms<'a>,
}

impl<'a> fmt::Display for GeneralFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.word, self.terms)
    }
}

/// `general_data`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GeneralData<'a> {
    Atomic(AtomicWord<'a>),
    Function(GeneralFunction<'a>),
    Variable(Variable<'a>),
    Number(Number<'a>),
    DistinctObject(DistinctObject<'a>),
    Formula(FormulaData<'a>),
}
impl_enum_anon_display!(
    GeneralData,
    Atomic,
    Function,
    Variable,
    Number,
    DistinctObject,
    Formula
);

/// `general_terms`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GeneralTerms<'a>(pub Vec<GeneralTerm<'a>>);

impl<'a> fmt::Display for GeneralTerms<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, ",", &self.0)
    }
}

/// `general_list`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GeneralList<'a>(pub Option<GeneralTerms<'a>>);

impl<'a> fmt::Display for GeneralList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            None => write!(f, "[]"),
            Some(terms) => write!(f, "[{}]", terms),
        }
    }
}

/// `general_term`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GeneralTerm<'a> {
    Data(GeneralData<'a>),
    Colon(GeneralData<'a>, Box<GeneralTerm<'a>>),
    List(GeneralList<'a>),
}

impl<'a> fmt::Display for GeneralTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GeneralTerm::*;
        match self {
            Data(d) => write!(f, "{}", d),
            Colon(d, t) => write!(f, "{}:{}", d, t),
            List(l) => write!(f, "{}", l),
        }
    }
}

/// `source`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Source<'a>(pub GeneralTerm<'a>);
impl_unit_anon_display!(Source);

/// `useful_info`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UsefulInfo<'a>(pub GeneralList<'a>);
impl_unit_anon_display!(UsefulInfo);

/// `optional_info`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct OptionalInfo<'a>(pub Option<UsefulInfo<'a>>);

impl<'a> fmt::Display for OptionalInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref info) => write!(f, ",{}", info),
            None => Ok(()),
        }
    }
}

/// `annotations`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Annotations<'a>(pub Option<(Source<'a>, OptionalInfo<'a>)>);

impl<'a> fmt::Display for Annotations<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some((source, info)) => write!(f, ",{}{}", source, info),
            None => Ok(()),
        }
    }
}

/// `fof_annotated`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FofAnnotated<'a> {
    pub name: Name<'a>,
    pub role: FormulaRole,
    pub formula: Box<FofFormula<'a>>,
    pub annotations: Box<Annotations<'a>>,
}

impl<'a> fmt::Display for FofAnnotated<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fof({},{},{}{}).",
            self.name, self.role, self.formula, self.annotations
        )
    }
}

/// `cnf_annotated`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CnfAnnotated<'a> {
    pub name: Name<'a>,
    pub role: FormulaRole,
    pub formula: Box<CnfFormula<'a>>,
    pub annotations: Box<Annotations<'a>>,
}

impl<'a> fmt::Display for CnfAnnotated<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cnf({},{},{}{}).",
            self.name, self.role, self.formula, self.annotations
        )
    }
}

/// `annotated_formula`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnnotatedFormula<'a> {
    Fof(FofAnnotated<'a>),
    Cnf(CnfAnnotated<'a>),
}
impl_enum_anon_display!(AnnotatedFormula, Fof, Cnf);

/// `file_name`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FileName<'a>(pub SingleQuoted<'a>);
impl_unit_anon_display!(FileName);

/// `name_list`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NameList<'a>(pub Vec<Name<'a>>);

impl<'a> fmt::Display for NameList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_list(f, ",", &self.0)
    }
}

/// `formula_selection`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FormulaSelection<'a>(pub Option<NameList<'a>>);

impl<'a> fmt::Display for FormulaSelection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(list) => write!(f, ",[{}]", list),
            None => Ok(()),
        }
    }
}

/// `include`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Include<'a> {
    pub file_name: FileName<'a>,
    pub selection: FormulaSelection<'a>,
}

impl<'a> fmt::Display for Include<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "include({}{}).", self.file_name, self.selection)
    }
}

/// `TPTP_input`
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TPTPInput<'a> {
    Annotated(AnnotatedFormula<'a>),
    Include(Include<'a>),
}
impl_enum_anon_display!(TPTPInput, Annotated, Include);
