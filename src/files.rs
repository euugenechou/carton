pub const PROJECT_BIN_MANIFEST: &str = r#"
[project]
name = "<PROJECT>"
version = "0.1.0"
type = "bin"
"#;

pub const PROJECT_LIB_MANIFEST: &str = r#"
[project]
name = "<PROJECT>"
version = "0.1.0"
type = "lib"
"#;

pub const PROJECT_BIN_BUILD: &str = r#"
project(
  '<PROJECT>',
  'c',
  version: '0.1.0',
)

subdir('src')
"#;

pub const PROJECT_LIB_BUILD: &str = r#"
project(
  '<PROJECT>',
  'c',
  version: '0.1.0',
)

include = include_directories('include')

subdir('include')
subdir('src')
subdir('tests')
"#;

pub const INCLUDE_BUILD: &str = r#"
install_headers('<PROJECT>.h')
"#;

pub const INCLUDE_TEMPLATE: &str = r#"
#pragma once

int add_two(int x, int y);
"#;

pub const SOURCE_BIN_BUILD: &str = r#"
src = [
    'main.c'
]

bin = executable(
  '<PROJECT>',
  src,
)
"#;

pub const SOURCE_BIN_TEMPLATE: &str = r#"
#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
    return 0;
}
"#;

pub const SOURCE_LIB_BUILD: &str = r#"
src = [
    'lib.c'
]
lib = library(
  '<PROJECT>',
  src,
  include_directories: include,
  install: true
)
"#;

pub const SOURCE_LIB_TEMPLATE: &str = r#"
#include <<PROJECT>.h>

int add_two(int x, int y) {
    return x + y;
}
"#;

pub const TEST_BUILD: &str = r#"
tester = executable(
  'tester',
  'test.c',
  include_directories: include,
  link_with: lib
)

test('it_works', tester)
"#;

pub const TEST_TEMPLATE: &str = r#"
#include <assert.h>
#include <<PROJECT>.h>

int main(void) {
    assert(add_two(1, 2) == 3);
    return 0;
}
"#;

pub const CLANG_FORMAT_STYLE: &str = r#"
AccessModifierOffset: -2
AlignAfterOpenBracket: AlwaysBreak
AlignConsecutiveMacros: false
AlignConsecutiveAssignments: false
AlignConsecutiveDeclarations: false
AlignEscapedNewlines: DontAlign
AlignOperands: false
AlignTrailingComments: false
AllowAllArgumentsOnNextLine: false
AllowAllConstructorInitializersOnNextLine: false
AllowAllParametersOfDeclarationOnNextLine: false
AllowShortBlocksOnASingleLine: false
AllowShortCaseLabelsOnASingleLine: false
AllowShortFunctionsOnASingleLine: Empty
AllowShortIfStatementsOnASingleLine: Never
AllowShortLambdasOnASingleLine: All
AllowShortLoopsOnASingleLine: false
AlwaysBreakAfterReturnType: None
AlwaysBreakBeforeMultilineStrings: true
AlwaysBreakTemplateDeclarations: Yes
BinPackArguments: false
BinPackParameters: false
BreakBeforeBinaryOperators: NonAssignment
BreakBeforeBraces: Attach
BreakBeforeTernaryOperators: true
BreakConstructorInitializers: AfterColon
BreakInheritanceList: AfterColon
BreakStringLiterals: false
ColumnLimit: 80
CompactNamespaces: false
ConstructorInitializerAllOnOneLineOrOnePerLine: true
ConstructorInitializerIndentWidth: 4
ContinuationIndentWidth: 4
Cpp11BracedListStyle: true
DerivePointerAlignment: false
FixNamespaceComments: true
IncludeBlocks: Regroup
IncludeCategories:
  - Regex:           '^<ext/.*\.h>'
    Priority:        2
    SortPriority:    0
    CaseSensitive:   false
  - Regex:           '^<.*\.h>'
    Priority:        1
    SortPriority:    0
    CaseSensitive:   false
  - Regex:           '^<.*'
    Priority:        2
    SortPriority:    0
    CaseSensitive:   false
  - Regex:           '.*'
    Priority:        3
    SortPriority:    0
    CaseSensitive:   false
IncludeIsMainRegex: '([-_](test|unittest))?$'
IndentCaseLabels: true
IndentPPDirectives: BeforeHash
IndentWidth: 4
IndentWrappedFunctionNames: false
KeepEmptyLinesAtTheStartOfBlocks: false
MaxEmptyLinesToKeep: 1
NamespaceIndentation: Inner
PointerAlignment: Left
ReflowComments: false
SortIncludes: true
SortUsingDeclarations: true
SpaceAfterCStyleCast: false
SpaceAfterLogicalNot: false
SpaceAfterTemplateKeyword: false
SpaceBeforeAssignmentOperators: true
SpaceBeforeCpp11BracedList: true
SpaceBeforeCtorInitializerColon: true
SpaceBeforeInheritanceColon: false
SpaceBeforeParens: ControlStatements
SpaceBeforeRangeBasedForLoopColon: true
SpaceInEmptyParentheses: false
SpacesBeforeTrailingComments: 2
SpacesInAngles: false
SpacesInCStyleCastParentheses: false
SpacesInContainerLiterals: false
SpacesInParentheses: false
SpacesInSquareBrackets: false
Standard: Cpp11
TabWidth: 4
UseTab: Never
"#;

pub const GITIGNORE: &str = r#"
/target
"#;
