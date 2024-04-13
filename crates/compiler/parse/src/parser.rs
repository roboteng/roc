use crate::state::State;
use bumpalo::collections::vec::Vec;
use bumpalo::Bump;
use roc_region::all::{Loc, Position, Region};
use Progress::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either<First, Second> {
    First(First),
    Second(Second),
}

impl<F: Copy, S: Copy> Copy for Either<F, S> {}

pub type ParseResult<'a, Output, Error> = Result<(Progress, Output, State<'a>), (Progress, Error)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    MadeProgress,
    NoProgress,
}

impl Progress {
    pub fn from_lengths(before: usize, after: usize) -> Self {
        Self::from_consumed(before - after)
    }
    pub fn from_consumed(chars_consumed: usize) -> Self {
        Self::progress_when(chars_consumed != 0)
    }

    pub fn progress_when(made_progress: bool) -> Self {
        if made_progress {
            Progress::MadeProgress
        } else {
            Progress::NoProgress
        }
    }

    pub fn or(&self, other: Self) -> Self {
        if (*self == MadeProgress) || (other == MadeProgress) {
            MadeProgress
        } else {
            NoProgress
        }
    }

    pub fn and(&self, other: Self) -> Self {
        if (*self == MadeProgress) && (other == MadeProgress) {
            MadeProgress
        } else {
            NoProgress
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError<'a> {
    Unexpected(Region),
    OutdentedTooFar,
    Eof(Region),
    InvalidPattern,
    BadUtf8,
    ReservedKeyword(Region),
    ArgumentsBeforeEquals(Region),
    NotYetImplemented(String),
    Todo,
    Type(EType<'a>),
    Pattern(EPattern<'a>),
    Expr(EExpr<'a>, Position),
    Header(EHeader<'a>),
    Space(BadInputError),
    NotEndOfFile(Position),
}
pub trait SpaceProblem: std::fmt::Debug {
    fn space_problem(e: BadInputError, pos: Position) -> Self;
}

macro_rules! impl_space_problem {
    ($($name:ident $(< $lt:tt >)?),*) => {
        $(
            impl $(< $lt >)? SpaceProblem for $name $(< $lt >)? {
                fn space_problem(e: BadInputError, pos: Position) -> Self {
                    Self::Space(e, pos)
                }
            }
        )*
    };
}

impl_space_problem! {
    EExpect<'a>,
    EExposes,
    EExpr<'a>,
    EGenerates,
    EGeneratesWith,
    EHeader<'a>,
    EIf<'a>,
    EImports,
    EInParens<'a>,
    EClosure<'a>,
    EList<'a>,
    EPackageEntry<'a>,
    EPackages<'a>,
    EPattern<'a>,
    EProvides<'a>,
    ERecord<'a>,
    ERequires<'a>,
    EString<'a>,
    EType<'a>,
    ETypeInParens<'a>,
    ETypeRecord<'a>,
    ETypeTagUnion<'a>,
    ETypedIdent<'a>,
    ETypeAbilityImpl<'a>,
    EWhen<'a>,
    EAbility<'a>,
    PInParens<'a>,
    PRecord<'a>,
    PList<'a>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EHeader<'a> {
    Provides(EProvides<'a>, Position),
    Exposes(EExposes, Position),
    Imports(EImports, Position),
    Requires(ERequires<'a>, Position),
    Packages(EPackages<'a>, Position),
    Generates(EGenerates, Position),
    GeneratesWith(EGeneratesWith, Position),

    Space(BadInputError, Position),
    Start(Position),
    ModuleName(Position),
    AppName(EString<'a>, Position),
    PackageName(EPackageName<'a>, Position),
    PlatformName(EPackageName<'a>, Position),
    IndentStart(Position),

    InconsistentModuleName(Region),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EProvides<'a> {
    Provides(Position),
    Open(Position),
    To(Position),
    IndentProvides(Position),
    IndentTo(Position),
    IndentListStart(Position),
    IndentPackage(Position),
    ListStart(Position),
    ListEnd(Position),
    Identifier(Position),
    Package(EPackageName<'a>, Position),
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EExposes {
    Exposes(Position),
    Open(Position),
    IndentExposes(Position),
    IndentListStart(Position),
    ListStart(Position),
    ListEnd(Position),
    Identifier(Position),
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ERequires<'a> {
    Requires(Position),
    Open(Position),
    IndentRequires(Position),
    IndentListStart(Position),
    ListStart(Position),
    ListEnd(Position),
    TypedIdent(ETypedIdent<'a>, Position),
    Rigid(Position),
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypedIdent<'a> {
    Space(BadInputError, Position),
    HasType(Position),
    IndentHasType(Position),
    Name(Position),
    Type(EType<'a>, Position),
    IndentType(Position),
    Identifier(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EPackages<'a> {
    Open(Position),
    Space(BadInputError, Position),
    Packages(Position),
    IndentPackages(Position),
    ListStart(Position),
    ListEnd(Position),
    IndentListStart(Position),
    IndentListEnd(Position),
    PackageEntry(EPackageEntry<'a>, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EPackageName<'a> {
    BadPath(EString<'a>, Position),
    Escapes(Position),
    Multiline(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EPackageEntry<'a> {
    BadPackage(EPackageName<'a>, Position),
    Shorthand(Position),
    Colon(Position),
    IndentPackage(Position),
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EImports {
    Open(Position),
    Imports(Position),
    IndentImports(Position),
    IndentListStart(Position),
    IndentListEnd(Position),
    ListStart(Position),
    ListEnd(Position),
    Identifier(Position),
    ExposingDot(Position),
    ShorthandDot(Position),
    Shorthand(Position),
    ModuleName(Position),
    Space(BadInputError, Position),
    IndentSetStart(Position),
    SetStart(Position),
    SetEnd(Position),
    TypedIdent(Position),
    AsKeyword(Position),
    StrLiteral(Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EGenerates {
    Open(Position),
    Generates(Position),
    IndentGenerates(Position),
    Identifier(Position),
    Space(BadInputError, Position),
    IndentTypeStart(Position),
    IndentTypeEnd(Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EGeneratesWith {
    Open(Position),
    With(Position),
    IndentWith(Position),
    IndentListStart(Position),
    IndentListEnd(Position),
    ListStart(Position),
    ListEnd(Position),
    Identifier(Position),
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadInputError {
    HasTab,
    HasMisplacedCarriageReturn,
    HasAsciiControl,
    BadUtf8,
}

impl<'a, T> SourceError<'a, T> {
    pub fn new(problem: T, state: &State<'a>) -> Self {
        Self {
            problem,
            bytes: state.original_bytes(),
        }
    }

    pub fn map_problem<E>(self, f: impl FnOnce(T) -> E) -> SourceError<'a, E> {
        SourceError {
            problem: f(self.problem),
            bytes: self.bytes,
        }
    }

    pub fn into_file_error(self, filename: std::path::PathBuf) -> FileError<'a, T> {
        FileError {
            problem: self,
            filename,
        }
    }
}

impl<'a> SyntaxError<'a> {
    pub fn into_source_error(self, state: &State<'a>) -> SourceError<'a, SyntaxError<'a>> {
        SourceError {
            problem: self,
            bytes: state.original_bytes(),
        }
    }

    pub fn into_file_error(
        self,
        filename: std::path::PathBuf,
        state: &State<'a>,
    ) -> FileError<'a, SyntaxError<'a>> {
        self.into_source_error(state).into_file_error(filename)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EExpr<'a> {
    TrailingOperator(Position),

    Start(Position),
    End(Position),
    BadExprEnd(Position),
    Space(BadInputError, Position),

    Dot(Position),
    Access(Position),
    UnaryNot(Position),
    UnaryNegate(Position),
    BadOperator(&'a str, Position),

    DefMissingFinalExpr(Position),
    DefMissingFinalExpr2(&'a EExpr<'a>, Position),
    Type(EType<'a>, Position),
    Pattern(&'a EPattern<'a>, Position),
    Ability(EAbility<'a>, Position),
    IndentDefBody(Position),
    IndentEquals(Position),
    IndentAnnotation(Position),
    Equals(Position),
    Colon(Position),
    DoubleColon(Position),
    Ident(Position),
    ElmStyleFunction(Region, Position),
    MalformedPattern(Position),
    QualifiedTag(Position),
    BackpassComma(Position),
    BackpassArrow(Position),

    When(EWhen<'a>, Position),
    If(EIf<'a>, Position),

    Expect(EExpect<'a>, Position),
    Dbg(EExpect<'a>, Position),

    Closure(EClosure<'a>, Position),
    Underscore(Position),
    Crash(Position),

    InParens(EInParens<'a>, Position),
    Record(ERecord<'a>, Position),
    OptionalValueInRecordBuilder(Region),
    RecordUpdateBuilder(Region),

    // SingleQuote errors are folded into the EString
    Str(EString<'a>, Position),

    Number(ENumber, Position),
    List(EList<'a>, Position),

    IndentStart(Position),
    IndentEnd(Position),

    UnexpectedComma(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ENumber {
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EString<'a> {
    Open(Position),

    CodePtOpen(Position),
    CodePtEnd(Position),

    InvalidSingleQuote(ESingleQuote, Position),

    Space(BadInputError, Position),
    EndlessSingleLine(Position),
    EndlessMultiLine(Position),
    EndlessSingleQuote(Position),
    UnknownEscape(Position),
    Format(&'a EExpr<'a>, Position),
    FormatEnd(Position),
    MultilineInsufficientIndent(Position),
    ExpectedDoubleQuoteGotSingleQuote(Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ESingleQuote {
    Empty,
    TooLong,
    InterpolationNotAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ERecord<'a> {
    End(Position),
    Open(Position),

    Updateable(Position),
    Field(Position),
    Colon(Position),
    QuestionMark(Position),
    Arrow(Position),
    Ampersand(Position),

    // TODO remove
    Expr(&'a EExpr<'a>, Position),

    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EInParens<'a> {
    End(Position),
    Open(Position),

    /// Empty parens, e.g. () is not allowed
    Empty(Position),

    ///
    Expr(&'a EExpr<'a>, Position),

    ///
    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EClosure<'a> {
    Space(BadInputError, Position),
    Start(Position),
    Arrow(Position),
    Comma(Position),
    Arg(Position),
    // TODO make EEXpr
    Pattern(EPattern<'a>, Position),
    Body(&'a EExpr<'a>, Position),
    IndentArrow(Position),
    IndentBody(Position),
    IndentArg(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EList<'a> {
    Open(Position),
    End(Position),
    Space(BadInputError, Position),

    Expr(&'a EExpr<'a>, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EWhen<'a> {
    Space(BadInputError, Position),
    When(Position),
    Is(Position),
    Pattern(EPattern<'a>, Position),
    Arrow(Position),
    Bar(Position),

    IfToken(Position),
    IfGuard(&'a EExpr<'a>, Position),

    Condition(&'a EExpr<'a>, Position),
    Branch(&'a EExpr<'a>, Position),

    IndentCondition(Position),
    IndentPattern(Position),
    IndentArrow(Position),
    IndentBranch(Position),
    IndentIfGuard(Position),
    PatternAlignment(u32, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EAbility<'a> {
    Space(BadInputError, Position),
    Type(EType<'a>, Position),

    DemandAlignment(i32, Position),
    DemandName(Position),
    DemandColon(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EIf<'a> {
    Space(BadInputError, Position),
    If(Position),
    Then(Position),
    Else(Position),
    // TODO make EEXpr
    Condition(&'a EExpr<'a>, Position),
    ThenBranch(&'a EExpr<'a>, Position),
    ElseBranch(&'a EExpr<'a>, Position),

    IndentCondition(Position),
    IndentIf(Position),
    IndentThenToken(Position),
    IndentElseToken(Position),
    IndentThenBranch(Position),
    IndentElseBranch(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EExpect<'a> {
    Space(BadInputError, Position),
    Dbg(Position),
    Expect(Position),
    Condition(&'a EExpr<'a>, Position),
    Continuation(&'a EExpr<'a>, Position),
    IndentCondition(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EPattern<'a> {
    Record(PRecord<'a>, Position),
    List(PList<'a>, Position),
    AsKeyword(Position),
    AsIdentifier(Position),
    Underscore(Position),
    NotAPattern(Position),

    Start(Position),
    End(Position),
    Space(BadInputError, Position),

    PInParens(PInParens<'a>, Position),
    NumLiteral(ENumber, Position),

    IndentStart(Position),
    IndentEnd(Position),
    AsIndentStart(Position),

    AccessorFunction(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PRecord<'a> {
    End(Position),
    Open(Position),

    Field(Position),
    Colon(Position),
    Optional(Position),

    Pattern(&'a EPattern<'a>, Position),
    Expr(&'a EExpr<'a>, Position),

    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PList<'a> {
    End(Position),
    Open(Position),

    Rest(Position),
    Pattern(&'a EPattern<'a>, Position),

    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PInParens<'a> {
    Empty(Position),
    End(Position),
    Open(Position),
    Pattern(&'a EPattern<'a>, Position),

    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EType<'a> {
    Space(BadInputError, Position),

    UnderscoreSpacing(Position),
    TRecord(ETypeRecord<'a>, Position),
    TTagUnion(ETypeTagUnion<'a>, Position),
    TInParens(ETypeInParens<'a>, Position),
    TApply(ETypeApply, Position),
    TInlineAlias(ETypeInlineAlias, Position),
    TBadTypeVariable(Position),
    TWildcard(Position),
    TInferred(Position),
    ///
    TStart(Position),
    TEnd(Position),
    TFunctionArgument(Position),
    TWhereBar(Position),
    TImplementsClause(Position),
    TAbilityImpl(ETypeAbilityImpl<'a>, Position),
    ///
    TIndentStart(Position),
    TIndentEnd(Position),
    TAsIndentStart(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeRecord<'a> {
    End(Position),
    Open(Position),

    Field(Position),
    Colon(Position),
    Optional(Position),
    Type(&'a EType<'a>, Position),

    Space(BadInputError, Position),

    IndentOpen(Position),
    IndentColon(Position),
    IndentOptional(Position),
    IndentEnd(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeTagUnion<'a> {
    End(Position),
    Open(Position),

    Type(&'a EType<'a>, Position),

    Space(BadInputError, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeInParens<'a> {
    /// e.g. (), which isn't a valid type
    Empty(Position),

    End(Position),
    Open(Position),
    ///
    Type(&'a EType<'a>, Position),

    ///
    Space(BadInputError, Position),
    ///
    IndentOpen(Position),
    IndentEnd(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeApply {
    ///
    StartNotUppercase(Position),
    End(Position),
    Space(BadInputError, Position),
    ///
    DoubleDot(Position),
    TrailingDot(Position),
    StartIsNumber(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeInlineAlias {
    NotAnAlias(Position),
    Qualified(Position),
    ArgumentNotLowercase(Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ETypeAbilityImpl<'a> {
    End(Position),
    Open(Position),

    Field(Position),
    Colon(Position),
    Arrow(Position),
    Optional(Position),
    Type(&'a EType<'a>, Position),

    Space(BadInputError, Position),

    Updateable(Position),
    QuestionMark(Position),
    Ampersand(Position),
    Expr(&'a EExpr<'a>, Position),
    IndentBar(Position),
    IndentAmpersand(Position),
}

impl<'a> From<ERecord<'a>> for ETypeAbilityImpl<'a> {
    fn from(e: ERecord<'a>) -> Self {
        match e {
            ERecord::End(p) => ETypeAbilityImpl::End(p),
            ERecord::Open(p) => ETypeAbilityImpl::Open(p),
            ERecord::Field(p) => ETypeAbilityImpl::Field(p),
            ERecord::Colon(p) => ETypeAbilityImpl::Colon(p),
            ERecord::Arrow(p) => ETypeAbilityImpl::Arrow(p),
            ERecord::Space(s, p) => ETypeAbilityImpl::Space(s, p),
            ERecord::Updateable(p) => ETypeAbilityImpl::Updateable(p),
            ERecord::QuestionMark(p) => ETypeAbilityImpl::QuestionMark(p),
            ERecord::Ampersand(p) => ETypeAbilityImpl::Ampersand(p),
            ERecord::Expr(e, p) => ETypeAbilityImpl::Expr(e, p),
        }
    }
}

#[derive(Debug)]
pub struct SourceError<'a, T> {
    pub problem: T,
    pub bytes: &'a [u8],
}

#[derive(Debug)]
pub struct FileError<'a, T> {
    pub problem: SourceError<'a, T>,
    pub filename: std::path::PathBuf,
}

pub trait Parser<'a, Output, Error> {
    fn parse(
        &self,
        arena: &'a Bump,
        state: State<'a>,
        min_indent: u32,
    ) -> ParseResult<'a, Output, Error>;

    #[cfg(not(feature = "parse_debug_trace"))]
    #[inline(always)]
    fn trace(self, _message: &'static str) -> Self
    where
        Self: Sized,
        Output: std::fmt::Debug,
        Error: std::fmt::Debug,
    {
        self
    }

    #[cfg(feature = "parse_debug_trace")]
    fn trace(self, message: &'static str) -> Traced<'a, Output, Error, Self>
    where
        Self: Sized,
        Output: std::fmt::Debug,
        Error: std::fmt::Debug,
    {
        Traced {
            parser: self,
            message,
            _phantom: Default::default(),
        }
    }
}

impl<'a, F, Output, Error> Parser<'a, Output, Error> for F
where
    Error: 'a,
    F: Fn(&'a Bump, State<'a>, u32) -> ParseResult<'a, Output, Error>,
{
    fn parse(
        &self,
        arena: &'a Bump,
        state: State<'a>,
        min_indent: u32,
    ) -> ParseResult<'a, Output, Error> {
        self(arena, state, min_indent)
    }
}

#[cfg(feature = "parse_debug_trace")]
pub struct Traced<'a, O, E, P: Parser<'a, O, E>> {
    parser: P,
    message: &'static str,
    _phantom: std::marker::PhantomData<&'a (O, E)>,
}

#[cfg(feature = "parse_debug_trace")]
impl<'a, O: std::fmt::Debug, E: std::fmt::Debug, P: Parser<'a, O, E>> Parser<'a, O, E>
    for Traced<'a, O, E, P>
where
    E: 'a,
{
    fn parse(&self, arena: &'a Bump, state: State<'a>, min_indent: u32) -> ParseResult<'a, O, E> {
        use std::cell::RefCell;

        thread_local! {
            pub static INDENT: RefCell<usize> = RefCell::new(0);
        }

        // This should be enough for anyone. Right? RIGHT?
        let indent_text =
            "| ; : ! | ; : ! | ; : ! | ; : ! | ; : ! | ; : ! | ; : ! | ; : ! | ; : ! ";

        let cur_indent = INDENT.with(|i| *i.borrow());

        println!(
            "{:<5?}: {}{:<50}",
            state.pos(),
            &indent_text[..cur_indent * 2],
            self.message
        );

        let previous_state = state.clone();
        INDENT.with(|i| *i.borrow_mut() += 1);
        let res = self.parser.parse(arena, state, min_indent);
        INDENT.with(|i| *i.borrow_mut() = cur_indent);

        let (progress, value, state) = match &res {
            Ok((progress, result, state)) => (progress, Ok(result), state),
            Err((progress, error)) => (progress, Err(error), &previous_state),
        };

        println!(
            "{:<5?}: {}{:<50} {:<15} {:?}",
            state.pos(),
            &indent_text[..cur_indent * 2],
            self.message,
            format!("{:?}", progress),
            value
        );

        res
    }
}

/// Allocates the output of the given parser and returns a reference to it.
/// This also happens if the given parser fails.
///
/// # Examples
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, allocated, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser_inner = word("hello", Problem::NotFound);
/// let alloc_parser = allocated(parser_inner);
///
/// // Success case
/// let (progress, output, state) = alloc_parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, &());
/// assert_eq!(state.pos(), Position::new(5));
///
/// // Error case
/// let (progress, err) = alloc_parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
pub fn allocated<'a, P, Val, Error>(parser: P) -> impl Parser<'a, &'a Val, Error>
where
    Error: 'a,
    P: Parser<'a, Val, Error>,
    Val: 'a,
{
    move |arena, state: State<'a>, min_indent: u32| {
        let (progress, answer, state) = parser.parse(arena, state, min_indent)?;

        Ok((progress, &*arena.alloc(answer), state))
    }
}

/// Apply transform function to turn output of given parser into another parser.
/// Can be used to chain two parsers.
///
/// # Examples
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, and_then, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser1 = word("hello", Problem::NotFound);
/// let parser = and_then(parser1, move |p,b| word(", ", Problem::NotFound));
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(7));
///
/// // Error case
/// let (progress, problem) = parser.parse(&arena, State::new("hello!! world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(problem, Problem::NotFound(Position::new(5)));
/// ```
pub fn and_then<'a, P1, P2, F, Before, After, Error>(
    parser: P1,
    transform: F,
) -> impl Parser<'a, After, Error>
where
    P1: Parser<'a, Before, Error>,
    P2: Parser<'a, After, Error>,
    F: Fn(Progress, Before) -> P2,
    Error: 'a,
{
    move |arena, state, min_indent| {
        parser
            .parse(arena, state, min_indent)
            .and_then(|(progress, output, next_state)| {
                transform(progress, output).parse(arena, next_state, min_indent)
            })
    }
}

/// Creates a new parser that can change its output based on a function.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, then, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// #     OddChar(Position),
/// # }
/// # let arena = Bump::new();
/// let parser_inner = word("hello", Problem::NotFound);
///
/// let parser = then(parser_inner,
///     |arena, new_state, progress, output| {
///         // arbitrary check
///         if new_state.pos().offset % 2 == 0 {
///             Ok((progress, output, new_state))
///         } else {
///             Err((Progress::NoProgress, Problem::OddChar(new_state.pos())))
///         }
///     }
/// );
///
/// let actual = parser.parse(&arena, State::new("hello, world".as_bytes()), 0);
/// assert!(actual.is_err());
/// ```
pub fn then<'a, P1, F, Before, After, E>(parser: P1, transform: F) -> impl Parser<'a, After, E>
where
    P1: Parser<'a, Before, E>,
    After: 'a,
    E: 'a,
    F: Fn(&'a Bump, State<'a>, Progress, Before) -> ParseResult<'a, After, E>,
{
    move |arena, state, min_indent| {
        parser
            .parse(arena, state, min_indent)
            .and_then(|(progress, output, next_state)| {
                transform(arena, next_state, progress, output)
            })
    }
}

/// Matches a word/string exactly, useful when finding a keyword.
/// This only matches if the next char is whitespace, the start of a comment, or the end of a line.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, keyword};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = keyword("when", Problem::NotFound);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("when".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 4);
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("whence".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
pub fn keyword<'a, ToError, E>(
    keyword_str: &'static str,
    if_error: ToError,
) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    move |_, mut state: State<'a>, _min_indent| {
        let width = keyword_str.len();

        if !state.bytes().starts_with(keyword_str.as_bytes()) {
            return Err((NoProgress, if_error(state.pos())));
        }

        // the next character should not be an identifier character
        // to prevent treating `whence` or `iffy` as keywords
        match state.bytes().get(width) {
            Some(next) if *next == b' ' || *next == b'#' || *next == b'\n' || *next == b'\r' => {
                state = state.advance(width);
                Ok((MadeProgress, (), state))
            }
            None => {
                state = state.advance(width);
                Ok((MadeProgress, (), state))
            }
            Some(_) => Err((NoProgress, if_error(state.pos()))),
        }
    }
}

/// Parse zero or more values separated by a delimiter (e.g. a comma) whose
/// values are discarded
pub fn sep_by0<'a, P, D, Val, Error>(
    delimiter: D,
    parser: P,
) -> impl Parser<'a, Vec<'a, Val>, Error>
where
    D: Parser<'a, (), Error>,
    P: Parser<'a, Val, Error>,
    Error: 'a,
{
    move |arena, state: State<'a>, min_indent: u32| {
        let original_state = state.clone();

        let start_bytes_len = state.bytes().len();

        match parser.parse(arena, state, min_indent) {
            Ok((elem_progress, first_output, next_state)) => {
                // in practice, we want elements to make progress
                debug_assert_eq!(elem_progress, MadeProgress);

                let mut state = next_state;
                let mut buf = Vec::with_capacity_in(1, arena);

                buf.push(first_output);

                loop {
                    match delimiter.parse(arena, state.clone(), min_indent) {
                        Ok((_, (), next_state)) => {
                            // If the delimiter passed, check the element parser.
                            match parser.parse(arena, next_state.clone(), min_indent) {
                                Ok((element_progress, next_output, next_state)) => {
                                    // in practice, we want elements to make progress
                                    debug_assert_eq!(element_progress, MadeProgress);

                                    state = next_state;
                                    buf.push(next_output);
                                }
                                Err((_, fail)) => {
                                    // If the delimiter parsed, but the following
                                    // element did not, that's a fatal error.
                                    let progress = Progress::from_lengths(
                                        start_bytes_len,
                                        next_state.bytes().len(),
                                    );

                                    return Err((progress, fail));
                                }
                            }
                        }
                        Err((delim_progress, fail)) => match delim_progress {
                            MadeProgress => return Err((MadeProgress, fail)),
                            NoProgress => return Ok((NoProgress, buf, state)),
                        },
                    }
                }
            }
            Err((element_progress, fail)) => match element_progress {
                MadeProgress => Err((MadeProgress, fail)),
                NoProgress => Ok((NoProgress, Vec::new_in(arena), original_state)),
            },
        }
    }
}

/// Parse zero or more values separated by a delimiter (e.g. a comma)
/// with an optional trailing delimiter whose values are discarded
pub fn trailing_sep_by0<'a, P, D, Val, Error>(
    delimiter: D,
    parser: P,
) -> impl Parser<'a, Vec<'a, Val>, Error>
where
    D: Parser<'a, (), Error>,
    P: Parser<'a, Val, Error>,
    Error: 'a,
{
    move |arena, state: State<'a>, min_indent: u32| {
        let original_state = state.clone();
        let start_bytes_len = state.bytes().len();

        match parser.parse(arena, state, min_indent) {
            Ok((progress, first_output, next_state)) => {
                // in practice, we want elements to make progress
                debug_assert_eq!(progress, MadeProgress);
                let mut state = next_state;
                let mut buf = Vec::with_capacity_in(1, arena);

                buf.push(first_output);

                loop {
                    match delimiter.parse(arena, state.clone(), min_indent) {
                        Ok((_, (), next_state)) => {
                            // If the delimiter passed, check the element parser.
                            match parser.parse(arena, next_state.clone(), min_indent) {
                                Ok((element_progress, next_output, next_state)) => {
                                    // in practice, we want elements to make progress
                                    debug_assert_eq!(element_progress, MadeProgress);

                                    state = next_state;
                                    buf.push(next_output);
                                }
                                Err((_, _fail)) => {
                                    // If the delimiter parsed, but the following
                                    // element did not, that means we saw a trailing comma
                                    let progress = Progress::from_lengths(
                                        start_bytes_len,
                                        next_state.bytes().len(),
                                    );
                                    return Ok((progress, buf, next_state));
                                }
                            }
                        }
                        Err((delim_progress, fail)) => match delim_progress {
                            MadeProgress => return Err((MadeProgress, fail)),
                            NoProgress => return Ok((NoProgress, buf, state)),
                        },
                    }
                }
            }
            Err((element_progress, fail)) => match element_progress {
                MadeProgress => Err((MadeProgress, fail)),
                NoProgress => Ok((NoProgress, Vec::new_in(arena), original_state)),
            },
        }
    }
}

/// Parse one or more values separated by a delimiter (e.g. a comma) whose
/// values are discarded
pub fn sep_by1<'a, P, D, Val, Error>(
    delimiter: D,
    parser: P,
) -> impl Parser<'a, Vec<'a, Val>, Error>
where
    D: Parser<'a, (), Error>,
    P: Parser<'a, Val, Error>,
    Error: 'a,
{
    move |arena, state: State<'a>, min_indent: u32| {
        let start_bytes_len = state.bytes().len();

        match parser.parse(arena, state, min_indent) {
            Ok((progress, first_output, next_state)) => {
                debug_assert_eq!(progress, MadeProgress);
                let mut state = next_state;
                let mut buf = Vec::with_capacity_in(1, arena);

                buf.push(first_output);

                loop {
                    let old_state = state.clone();
                    match delimiter.parse(arena, state, min_indent) {
                        Ok((_, (), next_state)) => {
                            // If the delimiter passed, check the element parser.
                            match parser.parse(arena, next_state, min_indent) {
                                Ok((_, next_output, next_state)) => {
                                    state = next_state;
                                    buf.push(next_output);
                                }
                                Err((_, fail)) => {
                                    return Err((MadeProgress, fail));
                                }
                            }
                        }
                        Err((delim_progress, fail)) => {
                            match delim_progress {
                                MadeProgress => {
                                    // fail if the delimiter made progress
                                    return Err((MadeProgress, fail));
                                }
                                NoProgress => {
                                    let progress = Progress::from_lengths(
                                        start_bytes_len,
                                        old_state.bytes().len(),
                                    );
                                    return Ok((progress, buf, old_state));
                                }
                            }
                        }
                    }
                }
            }
            Err((fail_progress, fail)) => Err((fail_progress, fail)),
        }
    }
}

/// Parse one or more values separated by a delimiter (e.g. a comma) whose
/// values are discarded
pub fn sep_by1_e<'a, P, V, D, Val, Error>(
    delimiter: D,
    parser: P,
    to_element_error: V,
) -> impl Parser<'a, Vec<'a, Val>, Error>
where
    D: Parser<'a, (), Error>,
    P: Parser<'a, Val, Error>,
    V: Fn(Position) -> Error,
    Error: 'a,
{
    move |arena, state: State<'a>, min_indent: u32| {
        let original_state = state.clone();
        let start_bytes_len = state.bytes().len();

        match parser.parse(arena, state, min_indent) {
            Ok((progress, first_output, next_state)) => {
                debug_assert_eq!(progress, MadeProgress);
                let mut state = next_state;
                let mut buf = Vec::with_capacity_in(1, arena);

                buf.push(first_output);

                loop {
                    let old_state = state.clone();
                    match delimiter.parse(arena, state, min_indent) {
                        Ok((_, (), next_state)) => {
                            // If the delimiter passed, check the element parser.
                            match parser.parse(arena, next_state.clone(), min_indent) {
                                Ok((_, next_output, next_state)) => {
                                    state = next_state;
                                    buf.push(next_output);
                                }
                                Err((MadeProgress, fail)) => {
                                    return Err((MadeProgress, fail));
                                }
                                Err((NoProgress, _fail)) => {
                                    return Err((NoProgress, to_element_error(next_state.pos())));
                                }
                            }
                        }
                        Err((delim_progress, fail)) => {
                            match delim_progress {
                                MadeProgress => {
                                    // fail if the delimiter made progress
                                    return Err((MadeProgress, fail));
                                }
                                NoProgress => {
                                    let progress = Progress::from_lengths(
                                        start_bytes_len,
                                        old_state.bytes().len(),
                                    );
                                    return Ok((progress, buf, old_state));
                                }
                            }
                        }
                    }
                }
            }

            Err((MadeProgress, fail)) => Err((MadeProgress, fail)),
            Err((NoProgress, _fail)) => Err((NoProgress, to_element_error(original_state.pos()))),
        }
    }
}

/// Make the given parser optional, it can complete or not consume anything,
/// but it can't error with progress made.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, optional, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = optional(word("hello", Problem::NotFound));
///
/// // Parser completed case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, Some(()));
/// assert_eq!(state.pos().offset, 5);
///
/// // No progress case
/// let (progress, output, state) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(output, None);
/// assert_eq!(state.pos().offset, 0);
/// ```
pub fn optional<'a, P, T, E>(parser: P) -> impl Parser<'a, Option<T>, E>
where
    P: Parser<'a, T, E>,
    E: 'a,
{
    move |arena: &'a Bump, state: State<'a>, min_indent: u32| {
        // We have to clone this because if the optional parser fails,
        // we need to revert back to the original state.
        let original_state = state.clone();

        match parser.parse(arena, state, min_indent) {
            Ok((progress, out1, state)) => Ok((progress, Some(out1), state)),
            Err((MadeProgress, e)) => Err((MadeProgress, e)),
            Err((NoProgress, _)) => Ok((NoProgress, None, original_state)),
        }
    }
}

// MACRO COMBINATORS
//
// Using some combinators together results in combinatorial type explosion,
// this takes forever to compile. Using macros instead avoids this!

/// Wraps the output of the given parser in a [`Loc`](../roc_region/all/struct.Loc.html) struct,
/// to provide location information.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_parse::loc;
/// # use roc_region::all::{Loc, Position};
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = loc!(word("hello", Problem::NotFound));
///
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, Loc::new(0, 5, ()));
/// assert_eq!(state.pos().offset, 5);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! loc {
    ($parser:expr) => {
        move |arena, state: $crate::state::State<'a>, min_indent: u32| {
            use roc_region::all::{Loc, Region};

            let start = state.pos();

            match $parser.parse(arena, state, min_indent) {
                Ok((progress, value, state)) => {
                    let end = state.pos();
                    let region = Region::new(start, end);

                    Ok((progress, Loc { region, value }, state))
                }
                Err(err) => Err(err),
            }
        }
    };
}

/// If the first one parses, ignore its output and move on to parse with the second one.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_parse::ident::lowercase_ident;
/// # use roc_parse::skip_first;
/// # use bumpalo::Bump;
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = skip_first!(
///    word("hello, ", |_| ()),
///    lowercase_ident()
/// );
///
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, "world");
/// assert_eq!(state.pos().offset, 12);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! skip_first {
    ($p1:expr, $p2:expr) => {
        move |arena, state: $crate::state::State<'a>, min_indent: u32| match $p1
            .parse(arena, state, min_indent)
        {
            Ok((p1, _, state)) => match $p2.parse(arena, state, min_indent) {
                Ok((p2, out2, state)) => Ok((p1.or(p2), out2, state)),
                Err((p2, fail)) => Err((p1.or(p2), fail)),
            },
            Err((progress, fail)) => Err((progress, fail)),
        }
    };
}

/// If the first one parses, parse the second one; if it also parses, use the
/// output from the first one.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_parse::ident::lowercase_ident;
/// # use roc_parse::skip_second;
/// # use bumpalo::Bump;
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = skip_second!(
///    lowercase_ident(),
///    word(", world", |_| ())
/// );
///
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, "hello");
/// assert_eq!(state.pos().offset, 12);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! skip_second {
    ($p1:expr, $p2:expr) => {
        move |arena, state: $crate::state::State<'a>, min_indent: u32| match $p1
            .parse(arena, state, min_indent)
        {
            Ok((p1, out1, state)) => match $p2.parse(arena, state, min_indent) {
                Ok((p2, _, state)) => Ok((p1.or(p2), out1, state)),
                Err((p2, fail)) => Err((p1.or(p2), fail)),
            },
            Err((progress, fail)) => Err((progress, fail)),
        }
    };
}

#[macro_export]
macro_rules! collection_inner {
    ($elem:expr, $delimiter:expr, $space_before:expr) => {
        map_with_arena!(
            and!(
                and!(
                    $crate::blankspace::spaces(),
                    $crate::parser::trailing_sep_by0(
                        $delimiter,
                        $crate::blankspace::spaces_before_optional_after($elem,)
                    )
                ),
                $crate::blankspace::spaces()
            ),
            |arena: &'a bumpalo::Bump,
             ((spaces, mut parsed_elems), mut final_comments): (
                (
                    &'a [$crate::ast::CommentOrNewline<'a>],
                    bumpalo::collections::vec::Vec<'a, Loc<_>>
                ),
                &'a [$crate::ast::CommentOrNewline<'a>]
            )| {
                if !spaces.is_empty() {
                    if let Some(first) = parsed_elems.first_mut() {
                        first.value = $space_before(arena.alloc(first.value), spaces)
                    } else {
                        debug_assert!(final_comments.is_empty());
                        final_comments = spaces;
                    }
                }

                $crate::ast::Collection::with_items_and_comments(
                    arena,
                    parsed_elems.into_bump_slice(),
                    final_comments,
                )
            }
        )
    };
}

#[macro_export]
macro_rules! collection_trailing_sep_e {
    ($opening_brace:expr, $elem:expr, $delimiter:expr, $closing_brace:expr, $space_before:expr) => {
        between!(
            $opening_brace,
            $crate::parser::reset_min_indent($crate::collection_inner!(
                $elem,
                $delimiter,
                $space_before
            )),
            $closing_brace
        )
    };
}

/// Creates a parser that always succeeds with the given argument as output.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, Progress::NoProgress};
/// # use roc_parse::succeed;
/// # use bumpalo::Bump;
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = succeed!("different");
///
/// let (progress, output, state) = Parser::<&'a str,()>::parse(&parser, &arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(output, "different");
/// assert_eq!(state.pos().offset, 0);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! succeed {
    ($value:expr) => {
        move |_arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, _min_indent: u32| {
            Ok((NoProgress, $value, state))
        }
    };
}

/// Creates a parser that always fails.
/// If the given parser succeeds, the error is customized with the given function.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, fail_when};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// #     OtherProblem(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = fail_when(Problem::OtherProblem, word("hello", Problem::NotFound));
///
/// // When given parser succeeds:
/// let (progress, err) = Parser::<(), Problem>::parse(&parser, &arena, State::new("hello, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(err, Problem::OtherProblem(Position::new(0)));
///
/// // When given parser errors:
/// let (progress, err) = Parser::<(), Problem>::parse(&parser, &arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::new(0)));
/// ```
pub fn fail_when<'a, T, T2, E, F, P>(f: F, p: P) -> impl Parser<'a, T, E>
where
    T: 'a,
    T2: 'a,
    E: 'a,
    F: Fn(Position) -> E,
    P: Parser<'a, T2, E>,
{
    move |arena: &'a bumpalo::Bump, state: State<'a>, min_indent: u32| {
        let original_state = state.clone();
        match p.parse(arena, state, min_indent) {
            Ok((_, _, _)) => Err((MadeProgress, f(original_state.pos()))),
            Err((progress, err)) => Err((progress, err)),
        }
    }
}

/// Creates a parser that always fails using the given error function.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, fail};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = fail(Problem::NotFound);
///
/// let (progress, err) = Parser::<(), Problem>::parse(&parser, &arena, State::new("hello, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::new(0)));
/// ```
pub fn fail<'a, T, E, F>(f: F) -> impl Parser<'a, T, E>
where
    T: 'a,
    E: 'a,
    F: Fn(Position) -> E,
{
    move |_arena: &'a bumpalo::Bump, state: State<'a>, _min_indent: u32| {
        Err((NoProgress, f(state.pos())))
    }
}

/// Runs two parsers in succession. If both parsers succeed, the output is a tuple of both outputs.
/// Both parsers must have the same error type.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, byte};
/// # use roc_region::all::Position;
/// # use roc_parse::and;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = and!(
///     word("hello", Problem::NotFound),
///     byte(b',', Problem::NotFound)
/// );
///
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ((), ()));
/// assert_eq!(state.pos().offset, 6);
///
/// let (progress, err) = parser.parse(&arena, State::new("hello! world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(err, Problem::NotFound(Position::new(5)));
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! and {
    ($p1:expr, $p2:expr) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, min_indent: u32| match $p1
            .parse(arena, state, min_indent)
        {
            Ok((p1, out1, state)) => match $p2.parse(arena, state, min_indent) {
                Ok((p2, out2, state)) => Ok((p1.or(p2), (out1, out2), state)),
                Err((p2, fail)) => Err((p1.or(p2), fail)),
            },
            Err((progress, fail)) => Err((progress, fail)),
        }
    };
}

/// Take as input something that looks like a struct literal where values are parsers
/// and return a parser that runs each parser and returns a struct literal with the
/// results.
#[macro_export]
macro_rules! record {
    ($name:ident $(:: $name_ext:ident)* { $($field:ident: $parser:expr),* $(,)? }) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, min_indent: u32| {
            let mut state = state;
            let mut progress = NoProgress;
            $(
                let (new_progress, $field, new_state) = $parser.parse(arena, state, min_indent)?;
                state = new_state;
                progress = progress.or(new_progress);
            )*
            Ok((progress, $name $(:: $name_ext)* { $($field),* }, state))
        }
    };
}

/// Similar to [`and!`], but we modify the `min_indent` of the second parser to be
/// 1 greater than the `line_indent()` at the start of the first parser.
#[macro_export]
macro_rules! indented_seq {
    ($p1:expr, $p2:expr) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, _min_indent: u32| {
            let start_indent = state.line_indent();

            // TODO: we should account for min_indent here, but this doesn't currently work
            // because min_indent is sometimes larger than it really should be, which is in turn
            // due to uses of `increment_indent`.
            //
            // let p1_indent = std::cmp::max(start_indent, min_indent);

            let p1_indent = start_indent;
            let p2_indent = p1_indent + 1;

            match $p1.parse(arena, state, p1_indent) {
                Ok((p1, (), state)) => match $p2.parse(arena, state, p2_indent) {
                    Ok((p2, out2, state)) => Ok((p1.or(p2), out2, state)),
                    Err((p2, fail)) => Err((p1.or(p2), fail)),
                },
                Err((progress, fail)) => Err((progress, fail)),
            }
        }
    };
}

/// Similar to [`and!`], but we modify the `min_indent` of the second parser to be
/// 1 greater than the `column()` at the start of the first parser.
#[macro_export]
macro_rules! absolute_indented_seq {
    ($p1:expr, $p2:expr) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, _min_indent: u32| {
            let start_indent = state.column();

            let p1_indent = start_indent;
            let p2_indent = p1_indent + 1;

            match $p1.parse(arena, state, p1_indent) {
                Ok((p1, out1, state)) => match $p2.parse(arena, state, p2_indent) {
                    Ok((p2, out2, state)) => Ok((p1.or(p2), (out1, out2), state)),
                    Err((p2, fail)) => Err((p1.or(p2), fail)),
                },
                Err((progress, fail)) => Err((progress, fail)),
            }
        }
    };
}

/// Returns the result of the first parser that makes progress, even if it failed.
/// If no parsers make progress, the last parser's result is returned.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, byte};
/// # use roc_region::all::Position;
/// # use roc_parse::one_of;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser1 = one_of!(
///     word("hello", Problem::NotFound),
///     byte(b'h', Problem::NotFound)
/// );
/// let (progress, output, state) = parser1.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 5);
///
/// let parser2 = one_of!(
///     // swapped the order of the parsers
///     byte(b'h', Problem::NotFound),
///     word("hello", Problem::NotFound)
/// );
/// let (progress, output, state) = parser2.parse(&arena, State::new("hello! world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 1);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! one_of {
    ($p1:expr, $p2:expr) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, min_indent: u32| {
            let original_state = state.clone();

            match $p1.parse(arena, state, min_indent) {
                valid @ Ok(_) => valid,
                Err((MadeProgress, fail)) => Err((MadeProgress, fail)),
                Err((NoProgress, _)) => $p2.parse(arena, original_state, min_indent),
            }
        }
    };

    ($p1:expr, $($others:expr),+) => {
        one_of!($p1, one_of!($($others),+))
    };
    ($p1:expr, $($others:expr),+ $(,)?) => {
        one_of!($p1, $($others),+)
    };
}

pub fn reset_min_indent<'a, P, T, X: 'a>(parser: P) -> impl Parser<'a, T, X>
where
    P: Parser<'a, T, X>,
{
    move |arena, state, _min_indent| parser.parse(arena, state, 0)
}

pub fn set_min_indent<'a, P, T, X: 'a>(min_indent: u32, parser: P) -> impl Parser<'a, T, X>
where
    P: Parser<'a, T, X>,
{
    move |arena, state, _m| parser.parse(arena, state, min_indent)
}

pub fn increment_min_indent<'a, P, T, X: 'a>(parser: P) -> impl Parser<'a, T, X>
where
    P: Parser<'a, T, X>,
{
    move |arena, state, min_indent| parser.parse(arena, state, min_indent + 1)
}

pub fn line_min_indent<'a, P, T, X: 'a>(parser: P) -> impl Parser<'a, T, X>
where
    P: Parser<'a, T, X>,
{
    move |arena, state: State<'a>, min_indent| {
        let min_indent = std::cmp::max(state.line_indent(), min_indent);
        parser.parse(arena, state, min_indent)
    }
}

pub fn absolute_column_min_indent<'a, P, T, X: 'a>(parser: P) -> impl Parser<'a, T, X>
where
    P: Parser<'a, T, X>,
{
    move |arena, state: State<'a>, _min_indent| {
        let min_indent = state.column() + 1;
        parser.parse(arena, state, min_indent)
    }
}

/// Transforms a possible error, like `map_err` in Rust.
/// It has no effect if the given parser succeeds.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, specialize_err};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// #     Other(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = specialize_err(
///     |_prev_err, pos| Problem::Other(pos),
///     word("bye", Problem::NotFound)
/// );
///
/// let (progress, err) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::Other(Position::new(0)));
/// ```
pub fn specialize_err<'a, F, P, T, X, Y>(map_error: F, parser: P) -> impl Parser<'a, T, Y>
where
    F: Fn(X, Position) -> Y,
    P: Parser<'a, T, X>,
    Y: 'a,
{
    move |a, state: State<'a>, min_indent| {
        let original_state = state.clone();
        match parser.parse(a, state, min_indent) {
            Ok(t) => Ok(t),
            Err((p, error)) => Err((p, map_error(error, original_state.pos()))),
        }
    }
}

/// Transforms a possible error, like `map_err` in Rust.
/// Similar to [`specialize_err`], but the error is arena allocated, and the
/// mapping function receives a reference to the error.
/// It has no effect if the inner parser succeeds.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, specialize_err_ref};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// #     Other(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = specialize_err_ref(
///     |_prev_err, pos| Problem::Other(pos),
///     word("bye", Problem::NotFound)
/// );
///
/// let (progress, err) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::Other(Position::new(0)));
/// ```
pub fn specialize_err_ref<'a, F, P, T, X, Y>(map_error: F, parser: P) -> impl Parser<'a, T, Y>
where
    F: Fn(&'a X, Position) -> Y,
    P: Parser<'a, T, X>,
    Y: 'a,
    X: 'a,
{
    move |a, state: State<'a>, min_indent| {
        let original_state = state.clone();
        match parser.parse(a, state, min_indent) {
            Ok(t) => Ok(t),
            Err((p, error)) => Err((p, map_error(a.alloc(error), original_state.pos()))),
        }
    }
}

/// Matches an entire `str` and moves the state's position forward if it succeeds.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = word("hello", Problem::NotFound);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(5));
///
/// // Error case
/// let (progress, problem) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(problem, Problem::NotFound(Position::zero()));
/// ```
pub fn word<'a, ToError, E>(word: &'static str, to_error: ToError) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    debug_assert!(!word.contains('\n'));

    move |_arena: &'a Bump, state: State<'a>, _min_indent: u32| {
        if state.bytes().starts_with(word.as_bytes()) {
            let state = state.advance(word.len());
            Ok((MadeProgress, (), state))
        } else {
            Err((NoProgress, to_error(state.pos())))
        }
    }
}

/// Matches a single `u8` and moves the state's position forward if it succeeds.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, byte};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = byte(b'h', Problem::NotFound);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(1));
///
/// // Error case
/// let (progress, problem) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(problem, Problem::NotFound(Position::zero()));
/// ```
pub fn byte<'a, ToError, E>(byte_to_match: u8, to_error: ToError) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    debug_assert_ne!(byte_to_match, b'\n');

    move |_arena: &'a Bump, state: State<'a>, _min_indent: u32| match state.bytes().first() {
        Some(x) if *x == byte_to_match => {
            let state = state.advance(1);
            Ok((MadeProgress, (), state))
        }
        _ => Err((NoProgress, to_error(state.pos()))),
    }
}

/// Matches a single `u8` and moves the state's position forward if it succeeds.
/// This parser will fail if it is a lower indentation level than it should be.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, byte, byte_indent};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// #     WrongIndentLevel(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = byte_indent(b'h', Problem::WrongIndentLevel);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(1));
///
/// // Error case
/// let state = State::new(" hello, world".as_bytes());
/// let _ = byte(b' ', Problem::NotFound).parse(&arena, state.clone(), 0).unwrap();
/// let (progress, problem) = parser.parse(&arena, state, 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(problem, Problem::WrongIndentLevel(Position::zero()));
/// ```
pub fn byte_indent<'a, ToError, E>(byte_to_match: u8, to_error: ToError) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    debug_assert_ne!(byte_to_match, b'\n');

    move |_arena: &'a Bump, state: State<'a>, min_indent: u32| {
        if min_indent > state.column() {
            return Err((NoProgress, to_error(state.pos())));
        }

        match state.bytes().first() {
            Some(x) if *x == byte_to_match => {
                let state = state.advance(1);
                Ok((MadeProgress, (), state))
            }
            _ => Err((NoProgress, to_error(state.pos()))),
        }
    }
}

/// Matches two `u8` in a row.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, two_bytes};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = two_bytes(b'h', b'e', Problem::NotFound);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(2));
///
/// // Error case
/// let (progress, problem) = parser.parse(&arena, State::new("hi, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(problem, Problem::NotFound(Position::zero()));
/// ```
pub fn two_bytes<'a, ToError, E>(
    byte_1: u8,
    byte_2: u8,
    to_error: ToError,
) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    debug_assert_ne!(byte_1, b'\n');
    debug_assert_ne!(byte_2, b'\n');

    let needle = [byte_1, byte_2];

    move |_arena: &'a Bump, state: State<'a>, _min_indent: u32| {
        if state.bytes().starts_with(&needle) {
            let state = state.advance(2);
            Ok((MadeProgress, (), state))
        } else {
            Err((NoProgress, to_error(state.pos())))
        }
    }
}

/// Matches three `u8` in a row.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, three_bytes};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = three_bytes(b'h', b'e', b'l', Problem::NotFound);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos(), Position::new(3));
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("hi, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
pub fn three_bytes<'a, ToError, E>(
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    to_error: ToError,
) -> impl Parser<'a, (), E>
where
    ToError: Fn(Position) -> E,
    E: 'a,
{
    debug_assert_ne!(byte_1, b'\n');
    debug_assert_ne!(byte_2, b'\n');
    debug_assert_ne!(byte_3, b'\n');

    let needle = [byte_1, byte_2, byte_3];

    move |_arena: &'a Bump, state: State<'a>, _min_indent: u32| {
        if state.bytes().starts_with(&needle) {
            let state = state.advance(3);
            Ok((MadeProgress, (), state))
        } else {
            Err((NoProgress, to_error(state.pos())))
        }
    }
}

#[macro_export]
macro_rules! byte_check_indent {
    ($byte_to_match:expr, $problem:expr, $min_indent:expr, $indent_problem:expr) => {
        and!(
            byte($byte_to_match, $problem),
            $crate::parser::check_indent($min_indent, $indent_problem)
        )
    };
}

/// transform the `Ok` result of a parser
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_region::all::Position;
/// # use roc_parse::map;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = map!(
///     word("hello", Problem::NotFound),
///     |_output| "new output!"
/// );
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, "new output!");
/// assert_eq!(state.pos(), Position::new(5));
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
#[macro_export]
macro_rules! map {
    ($parser:expr, $transform:expr) => {
        move |arena, state, min_indent| {
            #[allow(clippy::redundant_closure_call)]
            $parser
                .parse(arena, state, min_indent)
                .map(|(progress, output, next_state)| (progress, $transform(output), next_state))
        }
    };
}

/// Maps/transforms the `Ok` result of parsing using the given function.
/// Similar to [`map!`], but the transform function also takes a bump allocator.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_region::all::Position;
/// # use roc_parse::map_with_arena;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = map_with_arena!(
///     word("hello", Problem::NotFound),
///     |_arena, _output| "new output!"
/// );
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, "new output!");
/// assert_eq!(state.pos(), Position::new(5));
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
#[macro_export]
macro_rules! map_with_arena {
    ($parser:expr, $transform:expr) => {
        move |arena, state, min_indent| {
            #[allow(clippy::redundant_closure_call)]
            $parser
                .parse(arena, state, min_indent)
                .map(|(progress, output, next_state)| {
                    (progress, $transform(arena, output), next_state)
                })
        }
    };
}

/// Applies the parser as many times as possible.
/// This parser will only fail if the given parser makes partial progress.
#[macro_export]
macro_rules! zero_or_more {
    ($parser:expr) => {
        move |arena, state: State<'a>, min_indent: u32| {
            use bumpalo::collections::Vec;

            let original_state = state.clone();

            let start_bytes_len = state.bytes().len();

            match $parser.parse(arena, state, min_indent) {
                Ok((_, first_output, next_state)) => {
                    let mut state = next_state;
                    let mut buf = Vec::with_capacity_in(1, arena);

                    buf.push(first_output);

                    loop {
                        let old_state = state.clone();
                        match $parser.parse(arena, state, min_indent) {
                            Ok((_, next_output, next_state)) => {
                                state = next_state;
                                buf.push(next_output);
                            }
                            Err((fail_progress, fail)) => {
                                match fail_progress {
                                    MadeProgress => {
                                        // made progress on an element and then failed; that's an error
                                        return Err((MadeProgress, fail));
                                    }
                                    NoProgress => {
                                        // the next element failed with no progress
                                        // report whether we made progress before
                                        let progress = Progress::from_lengths(start_bytes_len, old_state.bytes().len());
                                        return Ok((progress, buf, old_state));
                                    }
                                }
                            }
                        }
                    }
                }
                Err((fail_progress, fail)) => {
                    match fail_progress {
                        MadeProgress => {
                            // made progress on an element and then failed; that's an error
                            Err((MadeProgress, fail))
                        }
                        NoProgress => {
                            // the first element failed (with no progress), but that's OK
                            // because we only need to parse 0 elements
                            Ok((NoProgress, Vec::new_in(arena), original_state))
                        }
                    }
                }
            }
        }
    };
}

/// Creates a parser that matches one or more times.
/// Will fail if the given parser fails to match or matches partially.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, Progress::{MadeProgress, NoProgress}, word};
/// # use roc_region::all::Position;
/// # use roc_parse::one_or_more;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = one_or_more!(
///     word("hello, ", Problem::NotFound),
///     Problem::NotFound
/// );
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, bumpalo::vec![in &arena; (),()]);
/// assert_eq!(state.pos(), Position::new(14));
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! one_or_more {
    ($parser:expr, $to_error:expr) => {
        move |arena, state: State<'a>, min_indent: u32| {
            use bumpalo::collections::Vec;

            match $parser.parse(arena, state.clone(), min_indent) {
                Ok((_, first_output, next_state)) => {
                    let mut state = next_state;
                    let mut buf = Vec::with_capacity_in(1, arena);

                    buf.push(first_output);

                    loop {
                        let old_state = state.clone();
                        match $parser.parse(arena, state, min_indent) {
                            Ok((_, next_output, next_state)) => {
                                state = next_state;
                                buf.push(next_output);
                            }
                            Err((NoProgress, _)) => {
                                return Ok((MadeProgress, buf, old_state));
                            }
                            Err((MadeProgress, fail)) => {
                                return Err((MadeProgress, fail));
                            }
                        }
                    }
                }
                Err((progress, _)) => Err((progress, $to_error(state.pos()))),
            }
        }
    };
}

/// Creates a parser that debug prints the result of parsing.
/// It doesn't change the given parser at all,
/// useful for inspecting a parser during development.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word};
/// # use roc_region::all::Position;
/// # use roc_parse::debug;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = debug!(
///     word("hello", Problem::NotFound)
/// );
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 5);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! debug {
    ($parser:expr) => {
        move |arena, state: $crate::state::State<'a>, min_indent: u32| {
            dbg!($parser.parse(arena, state, min_indent))
        }
    };
}

/// Matches either of the two given parsers.
/// If the first parser succeeds, its result is used,
/// otherwise, the second parser's result is used.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, Progress::{MadeProgress, NoProgress}, word, Either};
/// # use roc_region::all::Position;
/// # use roc_parse::either;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = either!(
///     word("hello", Problem::NotFound),
///     word("bye", Problem::NotFound)
/// );
///
/// // Success cases
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, Either::First(()));
/// assert_eq!(state.pos().offset, 5);
///
/// let (progress, output, state) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, Either::Second(()));
/// assert_eq!(state.pos().offset, 3);
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("later, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! either {
    ($p1:expr, $p2:expr) => {
        move |arena: &'a bumpalo::Bump, state: $crate::state::State<'a>, min_indent: u32| {
            let original_state = state.clone();
            match $p1.parse(arena, state, min_indent) {
                Ok((progress, output, state)) => {
                    Ok((progress, $crate::parser::Either::First(output), state))
                }
                Err((NoProgress, _)) => {
                    match $p2.parse(arena, original_state.clone(), min_indent) {
                        Ok((progress, output, state)) => {
                            Ok((progress, $crate::parser::Either::Second(output), state))
                        }
                        Err((progress, fail)) => Err((progress, fail)),
                    }
                }
                Err((MadeProgress, fail)) => Err((MadeProgress, fail)),
            }
        }
    };
}

/// Given three parsers, parse them all but ignore the output of the first and last one.
/// Useful for parsing things between two braces (e.g. parentheses).
///
/// If any of the three parsers error, this will error.
///
/// # Example
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, byte};
/// # use roc_region::all::Position;
/// # use roc_parse::{between, skip_first, skip_second};
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = between!(
///     byte(b'(', Problem::NotFound),
///     word("hello", Problem::NotFound),
///     byte(b')', Problem::NotFound)
/// );
/// let (progress, output, state) = parser.parse(&arena, State::new("(hello), world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 7);
/// # }
/// # foo(&arena);
/// ```
#[macro_export]
macro_rules! between {
    ($opening_brace:expr, $parser:expr, $closing_brace:expr) => {
        skip_first!($opening_brace, skip_second!($parser, $closing_brace))
    };
}

/// Runs two parsers in succession. If both parsers succeed, the output is a tuple of both outputs.
/// Both parsers must have the same error type.
///
/// This is a function version of the [`and!`] macro.
/// For some reason, some usages won't compile unless they use this instead of the macro version.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, and, word};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser1 = word("hello", Problem::NotFound);
/// let parser2 = word(", ", Problem::NotFound);
/// let parser = and(parser1, parser2);
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, ((),()));
/// assert_eq!(state.pos(), Position::new(7));
///
/// // Error case
/// let (progress, err) = parser.parse(&arena, State::new("hello!! world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(err, Problem::NotFound(Position::new(5)));
/// ```
#[inline(always)]
pub fn and<'a, P1, P2, A, B, E>(p1: P1, p2: P2) -> impl Parser<'a, (A, B), E>
where
    P1: Parser<'a, A, E>,
    P2: Parser<'a, B, E>,
    P1: 'a,
    P2: 'a,
    A: 'a,
    B: 'a,
    E: 'a,
{
    and!(p1, p2)
}

/// Adds location info. This is a function version the [`loc!`] macro.
///
/// For some reason, some usages won't compile unless they use this instead of the macro version.
/// This is likely because the lifetime `'a` is not defined at the call site.
///
/// # Examples
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, loc};
/// # use roc_region::all::{Loc, Position};
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// # fn foo<'a>(arena: &'a Bump) {
/// let parser = loc(word("hello", Problem::NotFound));
///
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, Loc::new(0, 5, ()));
/// assert_eq!(state.pos().offset, 5);
/// # }
/// # foo(&arena);
/// ```
#[inline(always)]
pub fn loc<'a, P, Val, Error>(parser: P) -> impl Parser<'a, Loc<Val>, Error>
where
    P: Parser<'a, Val, Error>,
    Error: 'a,
{
    loc!(parser)
}

/// Maps/transforms the `Ok` result of parsing using the given function.
/// Similar to [`map!`], but the transform function also takes a bump allocator.
///
/// Function version of the [`map_with_arena!`] macro.
///
/// For some reason, some usages won't compile unless they use this instead of the macro version.
/// This is likely because the lifetime `'a` is not defined at the call site.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, map_with_arena};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = map_with_arena(
///     word("hello", Problem::NotFound),
///     |_arena, _output| "new output!"
/// );
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::MadeProgress);
/// assert_eq!(output, "new output!");
/// assert_eq!(state.pos(), Position::new(5));
///
/// // Error Case
/// let (progress, err) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
#[inline(always)]
pub fn map_with_arena<'a, P, F, Before, After, E>(
    parser: P,
    transform: F,
) -> impl Parser<'a, After, E>
where
    P: Parser<'a, Before, E>,
    P: 'a,
    F: Fn(&'a Bump, Before) -> After,
    F: 'a,
    Before: 'a,
    After: 'a,
    E: 'a,
{
    map_with_arena!(parser, transform)
}

/// Creates a new parser that does not progress but still forwards the output of
/// the given parser if it succeeds.
///
/// # Example
///
/// ```
/// # #![forbid(unused_imports)]
/// # use roc_parse::state::State;
/// # use crate::roc_parse::parser::{Parser, Progress, word, backtrackable};
/// # use roc_region::all::Position;
/// # use bumpalo::Bump;
/// # #[derive(Debug, PartialEq)]
/// # enum Problem {
/// #     NotFound(Position),
/// # }
/// # let arena = Bump::new();
/// let parser = backtrackable(
///     word("hello", Problem::NotFound),
/// );
///
/// // Success case
/// let (progress, output, state) = parser.parse(&arena, State::new("hello, world".as_bytes()), 0).unwrap();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(output, ());
/// assert_eq!(state.pos().offset, 5);
///
/// // Error Case
/// let (progress, err) = parser.parse(&arena, State::new("bye, world".as_bytes()), 0).unwrap_err();
/// assert_eq!(progress, Progress::NoProgress);
/// assert_eq!(err, Problem::NotFound(Position::zero()));
/// ```
pub fn backtrackable<'a, P, Val, Error>(parser: P) -> impl Parser<'a, Val, Error>
where
    P: Parser<'a, Val, Error>,
    Error: 'a,
{
    move |arena: &'a Bump, state: State<'a>, min_indent: u32| match parser
        .parse(arena, state, min_indent)
    {
        Ok((_, a, s1)) => Ok((NoProgress, a, s1)),
        Err((_, f)) => Err((NoProgress, f)),
    }
}
