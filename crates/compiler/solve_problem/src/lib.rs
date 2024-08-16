//! Provides types to describe problems that can occur during solving.
use std::{path::PathBuf, str::Utf8Error};

use roc_can::expected::{Expected, PExpected};
use roc_module::{
    ident::Lowercase,
    symbol::{ModuleId, Symbol},
};
use roc_problem::{can::CycleEntry, Severity};
use roc_region::all::Region;

use roc_types::types::{Category, ErrorType, PatternCategory};

#[derive(Debug, Clone)]
pub enum TypeError {
    BadExpr(Region, Category, ErrorType, Expected<ErrorType>),
    BadPattern(Region, PatternCategory, ErrorType, PExpected<ErrorType>),
    CircularType(Region, Symbol, ErrorType),
    CircularDef(Vec<CycleEntry>),
    UnexposedLookup(Region, Symbol),
    UnfulfilledAbility(Unfulfilled),
    BadExprMissingAbility(Region, Category, ErrorType, Vec<Unfulfilled>),
    BadPatternMissingAbility(Region, PatternCategory, ErrorType, Vec<Unfulfilled>),
    Exhaustive(roc_exhaustive::Error),
    StructuralSpecialization {
        region: Region,
        typ: ErrorType,
        ability: Symbol,
        member: Symbol,
    },
    WrongSpecialization {
        region: Region,
        ability_member: Symbol,
        expected_opaque: Symbol,
        found_opaque: Symbol,
    },
    IngestedFileBadUtf8(Box<PathBuf>, Utf8Error),
    IngestedFileUnsupportedType(Box<PathBuf>, ErrorType),
    UnexpectedModuleParams(Region, ModuleId),
    MissingModuleParams(Region, ModuleId, ErrorType),
    ModuleParamsMismatch(Region, ModuleId, ErrorType, ErrorType),
}

impl TypeError {
    pub fn severity(&self) -> Severity {
        use Severity::*;
        match self {
            TypeError::BadExpr(..) => RuntimeError,
            TypeError::BadPattern(..) => RuntimeError,
            TypeError::CircularType(..) => RuntimeError,
            TypeError::CircularDef(_) => RuntimeError,
            TypeError::UnexposedLookup(..) => RuntimeError,
            TypeError::UnfulfilledAbility(_) => RuntimeError,
            TypeError::BadExprMissingAbility(_, _, _, _) => RuntimeError,
            TypeError::BadPatternMissingAbility(_, _, _, _) => RuntimeError,
            // NB: if bidirectional exhaustiveness checking is implemented, the other direction
            // is also not a runtime error.
            TypeError::Exhaustive(exhtv) => exhtv.severity(),
            TypeError::StructuralSpecialization { .. } => RuntimeError,
            TypeError::WrongSpecialization { .. } => RuntimeError,
            TypeError::UnexpectedModuleParams(..) => Warning,
            TypeError::MissingModuleParams(..) => RuntimeError,
            TypeError::ModuleParamsMismatch(..) => RuntimeError,
            TypeError::IngestedFileBadUtf8(..) => Fatal,
            TypeError::IngestedFileUnsupportedType(..) => Fatal,
        }
    }

    pub fn region(&self) -> Option<Region> {
        match self {
            TypeError::BadExpr(region, ..)
            | TypeError::BadPattern(region, ..)
            | TypeError::CircularType(region, ..)
            | TypeError::UnexposedLookup(region, ..)
            | TypeError::BadExprMissingAbility(region, ..)
            | TypeError::StructuralSpecialization { region, .. }
            | TypeError::WrongSpecialization { region, .. }
            | TypeError::BadPatternMissingAbility(region, ..)
            | TypeError::UnexpectedModuleParams(region, ..)
            | TypeError::MissingModuleParams(region, ..)
            | TypeError::ModuleParamsMismatch(region, ..) => Some(*region),
            TypeError::UnfulfilledAbility(ab, ..) => ab.region(),
            TypeError::Exhaustive(e) => Some(e.region()),
            TypeError::CircularDef(c) => c.first().map(|ce| ce.symbol_region),
            TypeError::IngestedFileBadUtf8(_, _) => None,
            TypeError::IngestedFileUnsupportedType(_, _) => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Unfulfilled {
    /// No claimed implementation of an ability for an opaque type.
    OpaqueDoesNotImplement { typ: Symbol, ability: Symbol },
    /// Cannot derive implementation of an ability for a structural type.
    AdhocUnderivable {
        typ: ErrorType,
        ability: Symbol,
        reason: UnderivableReason,
    },
    /// Cannot derive implementation of an ability for an opaque type.
    OpaqueUnderivable {
        typ: ErrorType,
        ability: Symbol,
        opaque: Symbol,
        derive_region: Region,
        reason: UnderivableReason,
    },
}

impl Unfulfilled {
    fn region(&self) -> Option<Region> {
        match self {
            Unfulfilled::OpaqueDoesNotImplement { .. } => None,
            Unfulfilled::AdhocUnderivable { .. } => None,
            Unfulfilled::OpaqueUnderivable { derive_region, .. } => Some(*derive_region),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnderivableReason {
    NotABuiltin,
    /// The surface type is not derivable
    SurfaceNotDerivable(NotDerivableContext),
    /// A nested type is not derivable
    NestedNotDerivable(ErrorType, NotDerivableContext),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NotDerivableContext {
    NoContext,
    Function,
    UnboundVar,
    Opaque(Symbol),
    DecodeOptionalRecordField(Lowercase),
    Eq(NotDerivableEq),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NotDerivableEq {
    FloatingPoint,
}
