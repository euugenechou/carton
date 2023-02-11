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
