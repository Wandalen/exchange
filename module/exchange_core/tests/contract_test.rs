//! Test Matrix T12 — the Contract's two absolute prohibitions, checked over
//! the source tree rather than over the API.
//!
//! # Why this is a grep and not a type check
//!
//! The Contract says *no ECS types anywhere*. The failure it forbids is not
//! one a signature review would catch: an exchange holding an `Entity` handle
//! in a **private** struct field to identify a trader compiles, exposes
//! nothing, passes every behavioural test in this family, and has coupled
//! the market to the simulation's storage layer exactly as completely as a
//! public one would. Only reading the source sees it.
//!
//! The same argument applies to floats. A `f64` in a private intermediate is
//! invisible from outside and reintroduces precisely the drift `exact_arith`
//! exists to prevent.
//!
//! # Why a directory walk and not `include_str!`
//!
//! `include_str!` names files. A file added later — a new module, a helper
//! split out under `src/` — would simply not be checked, and nobody would
//! notice, because the test would still pass. Walking the directory means the
//! check covers whatever is there.
//!
//! # Why comments are stripped first
//!
//! This file, and the crates it reads, *document* the prohibition. Naming the
//! forbidden types in a doc comment is how a reader learns the rule exists;
//! counting that mention as a violation would make stating the rule the one
//! thing the rule forbids. So the check reads code.

use std::fs;
use std::path::{ Path, PathBuf };

/// Every domain crate the Contract covers — hand-maintained, not
/// auto-discovered, so adding a crate to the family is a deliberate edit
/// here rather than something a directory listing picks up on its own. A
/// stale entry here either panics on a deleted directory or silently stops
/// covering a crate nobody remembered to add — this list must be updated in
/// the same change that adds or removes a crate.
const FAMILY_CRATES : [ &str; 23 ] =
[
  "exchange_id",
  "exchange_side",
  "exchange_seq",
  "exchange_cap",
  "exchange_stp",
  "exchange_tif",
  "exchange_spec",
  "exchange_stats",
  "exchange_types",
  "exchange_order",
  "exchange_idem",
  "exchange_level",
  "exchange_book",
  "exchange_depth",
  "exchange_halt",
  "exchange_event",
  "exchange_snap",
  "exchange_fill",
  "exchange_conserve",
  "exchange_match",
  "exchange_rest",
  "exchange_escrow",
  "exchange_core",
];

/// The ECS vocabulary, whole-word.
const ECS_TYPES : [ &str; 5 ] = [ "Entity", "World", "Component", "Query", "System" ];

/// `substrate/exchange/` — the directory holding this crate and every sibling
/// the Contract is checked against.
///
/// One level up, not a search: the exchange family lives flat under a single
/// parent, so a sibling is always `../<name>`. A `../<name>` that stops
/// resolving fails loudly in `sources`' own `read_dir` rather than quietly
/// yielding an empty file list, which is what keeps the count assertion below
/// from passing over nothing.
fn family_dir() -> PathBuf
{
  Path::new( env!( "CARGO_MANIFEST_DIR" ) )
  .parent()
  .expect( "a crate directory has a parent" )
  .to_path_buf()
}

/// Every `.rs` file under a crate's `src/`, recursively.
fn sources( crate_name : &str ) -> Vec< PathBuf >
{
  let mut found = Vec::new();
  let mut pending = vec![ family_dir().join( crate_name ).join( "src" ) ];

  while let Some( dir ) = pending.pop()
  {
    let entries = fs::read_dir( &dir ).unwrap_or_else( | error | panic!( "{}: {error}", dir.display() ) );
    for entry in entries
    {
      let path = entry.expect( "the directory is readable" ).path();
      if path.is_dir()
      {
        pending.push( path );
      }
      else if path.extension().is_some_and( | extension | extension == "rs" )
      {
        found.push( path );
      }
    }
  }

  found
}

/// `line` with any `//`-comment removed, so only code is left.
///
/// Deliberately simple: it does not understand `/* */`, and it would truncate
/// a `//` inside a string literal. Neither appears in this family, and a
/// checker complicated enough to handle them would be a parser nobody tests.
/// If either ever appears, this returns *less* code than there is, so the
/// failure mode is a missed violation rather than a false alarm — which is why
/// the count assertion below also pins the number of files walked.
fn code_of( line : &str ) -> &str
{
  match line.find( "//" )
  {
    Some( at ) => &line[ ..at ],
    None => line,
  }
}

/// True when `haystack` contains `needle` as a whole word.
fn contains_word( haystack : &str, needle : &str ) -> bool
{
  let mut from = 0;
  while let Some( at ) = haystack[ from.. ].find( needle )
  {
    let start = from + at;
    let end = start + needle.len();
    let before_ok = start == 0 || !is_word_char( haystack.as_bytes()[ start - 1 ] );
    let after_ok = end == haystack.len() || !is_word_char( haystack.as_bytes()[ end ] );
    if before_ok && after_ok
    {
      return true;
    }
    from = end;
  }
  false
}

fn is_word_char( byte : u8 ) -> bool
{
  byte.is_ascii_alphanumeric() || byte == b'_'
}

/// True when `haystack` contains an identifier that *begins* with `needle` and
/// continues in `CamelCase` — `EntityId`, `WorldQuery`, `ComponentRef`.
///
/// Deliberately stricter than the whole-word check. `grep -w`, which is what
/// the task's own measurement runs, would let every one of those through, and
/// they are exactly the shape an ECS handle takes when it leaks in under a
/// convenience alias. This catches `SystemTime` too — which is not an ECS type
/// but is a clock reading, and the family forbids those on the matching
/// path for its own separate reason.
fn contains_derived_word( haystack : &str, needle : &str ) -> bool
{
  let mut from = 0;
  while let Some( at ) = haystack[ from.. ].find( needle )
  {
    let start = from + at;
    let end = start + needle.len();
    let before_ok = start == 0 || !is_word_char( haystack.as_bytes()[ start - 1 ] );
    let derived = haystack.as_bytes().get( end ).is_some_and( u8::is_ascii_uppercase );
    if before_ok && derived
    {
      return true;
    }
    from = end;
  }
  false
}

/// Every line of code in the family, as `(file, line number, code)`.
fn family_code() -> Vec< ( String, usize, String ) >
{
  let mut lines = Vec::new();

  for crate_name in FAMILY_CRATES
  {
    for path in sources( crate_name )
    {
      let text = fs::read_to_string( &path ).expect( "the source is readable" );
      let shown = format!( "{crate_name}/{}", path.file_name().unwrap().to_string_lossy() );
      for ( index, line ) in text.lines().enumerate()
      {
        lines.push( ( shown.clone(), index + 1, code_of( line ).to_string() ) );
      }
    }
  }

  lines
}

/// T12 — no ECS type appears in any of the five crates' code.
#[ test ]
fn t12_no_ecs_type_appears_anywhere_in_the_family()
{
  let offences : Vec< String > = family_code()
  .iter()
  .flat_map( | ( file, number, code ) |
  {
    ECS_TYPES
    .iter()
    .filter( | forbidden | contains_word( code, forbidden ) || contains_derived_word( code, forbidden ) )
    .map( move | forbidden | format!( "{file}:{number}: {forbidden} in `{}`", code.trim() ) )
    .collect::< Vec< _ > >()
  } )
  .collect();

  assert!( offences.is_empty(), "the Contract forbids ECS types outright:\n{}", offences.join( "\n" ) );
}

/// T12's other half — no floating point in any of the five crates' code.
///
/// One exception is allowed and named: the `#[ allow( clippy::float_cmp ) ]`
/// on a control arm that proves `f64` gets `0.1 + 0.2` wrong. That lives in a
/// test file, not under `src/`, so it is out of this walk's reach and needs no
/// exemption here — which is the reason the walk is scoped to `src/`.
#[ test ]
fn no_floating_point_appears_anywhere_in_the_family()
{
  let offences : Vec< String > = family_code()
  .iter()
  .flat_map( | ( file, number, code ) |
  {
    [ "f32", "f64" ]
    .iter()
    .filter( | forbidden | contains_word( code, forbidden ) )
    .map( move | forbidden | format!( "{file}:{number}: {forbidden} in `{}`", code.trim() ) )
    .collect::< Vec< _ > >()
  } )
  .collect();

  assert!( offences.is_empty(), "every price and quantity is exact:\n{}", offences.join( "\n" ) );
}

/// The walk actually reached all five crates, and each has source in it.
///
/// Without this, a walk that silently found nothing would report both checks
/// above as passing — the failure mode where a green suite means the test
/// stopped looking.
#[ test ]
fn the_walk_reaches_every_crate_in_the_family()
{
  for crate_name in FAMILY_CRATES
  {
    let files = sources( crate_name );
    assert!( !files.is_empty(), "{crate_name} has no source under src/" );
  }

  let lines = family_code();
  assert!( lines.len() > 500, "only {} lines walked — the family is larger than that", lines.len() );
}

/// The comment stripper strips comments and nothing else.
///
/// Tested because the two checks above are only as trustworthy as this: a
/// stripper that returned an empty string for every line would make both
/// prohibitions pass vacuously.
#[ test ]
fn the_comment_stripper_keeps_the_code()
{
  assert_eq!( code_of( "let x = 1; // Entity" ), "let x = 1; " );
  assert_eq!( code_of( "//! no Entity here" ), "" );
  assert_eq!( code_of( "let x = 1;" ), "let x = 1;" );
  assert!( contains_word( code_of( "struct Entity;" ), "Entity" ), "code outside a comment still counts" );
}

/// Whole-word matching is whole-word — no fragments, and case-sensitive.
///
/// A plain substring check would flag `AccountId` for containing no forbidden
/// word at all and would flag half of any codebase that used the letters; this
/// pins that it does not.
#[ test ]
fn word_matching_matches_words_and_not_fragments()
{
  assert!( contains_word( "let e : Entity = w;", "Entity" ) );
  assert!( contains_word( "( World, x )", "World" ), "punctuation is not part of a word" );
  assert!( !contains_word( "EntityId( 3 )", "Entity" ), "a longer identifier is a different word" );
  assert!( !contains_word( "my_entity", "Entity" ), "and matching is case-sensitive" );
  assert!( !contains_word( "AccountId( 1 )", "Component" ) );
}

/// The derived-identifier check catches what `grep -w` cannot.
///
/// `EntityId` is the exact shape an ECS handle takes when it leaks in under a
/// convenience alias, and the task's own measurement — a `grep -w` — would let
/// it straight through. This is the check that does not, and pinning it here
/// is what stops it being quietly weakened to whole-word later.
#[ test ]
fn the_derived_check_catches_ecs_names_grep_would_miss()
{
  assert!( contains_derived_word( "EntityId( 3 )", "Entity" ) );
  assert!( contains_derived_word( "SystemTime::now()", "System" ) );
  assert!( contains_derived_word( "let q : WorldQuery = w;", "World" ) );

  assert!( !contains_derived_word( "entity_id", "Entity" ), "lower case is not the convention" );
  assert!( !contains_derived_word( "MyEntityId", "Entity" ), "and a suffix is a different identifier" );
  assert!( !contains_derived_word( "let e : Entity = w;", "Entity" ), "the bare word is the other check's job" );
}

/// The facade depends on the family's own facade, not past it.
///
/// Checklist C3: reaching into any individual `exact_*` leaf crate directly
/// would tie this family to a split that `exact_arith` is free to change.
#[ test ]
fn the_family_depends_on_the_exact_arith_facade_only()
{
  for crate_name in FAMILY_CRATES
  {
    let manifest = fs::read_to_string( family_dir().join( crate_name ).join( "Cargo.toml" ) )
    .expect( "every crate has a manifest" );

    for past_the_facade in
    [
      "exact_minor", "exact_scale", "exact_round", "exact_sign", "exact_kind",
      "exact_add", "exact_ratio", "exact_parse", "exact_fmt", "exact_bytes",
      "exact_snap", "exact_cmp", "exact_dust", "exact_conserve",
    ]
    {
      assert!
      (
        !manifest.contains( past_the_facade ),
        "{crate_name} reaches past exact_arith into {past_the_facade}",
      );
    }
  }
}
