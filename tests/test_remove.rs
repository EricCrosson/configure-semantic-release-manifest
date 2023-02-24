use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

use configure_semantic_release_manifest::SemanticReleaseManifest;

fn check(initial: &str, to_remove: Vec<&str>, expected: &str) {
    let mut manifest = SemanticReleaseManifest::from_str(initial).unwrap();
    let result = manifest.remove_plugin_configuration(HashSet::from_iter(
        to_remove.into_iter().map(|s| s.to_owned()),
    ));
    assert!(result.is_ok());
    result.unwrap();
    assert_eq!(expected.trim(), manifest.to_string())
}

#[test]
fn should_remove_configuration_when_present() {
    check(
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release/github"
  ]
}            
        "#,
        vec!["@semantic-release/changelog"],
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}
        "#,
    )
}

#[test]
fn should_remove_complex_configuration_when_present() {
    check(
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    [
      "@semantic-release/github",
      {
        "assets": [
          {
            "path": "one",
            "label": "fish"
          },
          {
            "path": "two",
            "label": "fishes"
          }
        ]
      }
    ]
  ]
}            
        "#,
        vec!["@semantic-release/github"],
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog"
  ]
}
        "#,
    )
}

#[test]
fn should_remove_multiple_configurations_when_present() {
    check(
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release/github",
    "@semantic-release/git"
  ]
}            
        "#,
        vec!["@semantic-release/changelog", "@semantic-release/git"],
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}
        "#,
    )
}

#[test]
fn should_not_edit_configuration_when_not_present() {
    check(
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}            
        "#,
        vec!["@semantic-release/changelog"],
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}
        "#,
    )
}

#[test]
fn should_not_edit_unrelated_configuration_when_present() {
    check(
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/changelog",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}            
        "#,
        vec!["@semantic-release/changelog"],
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}
        "#,
    )
}

#[test]
fn should_not_edit_unrelated_configuration_when_not_present() {
    check(
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}            
        "#,
        vec!["@semantic-release/changelog"],
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/github"
  ]
}
        "#,
    )
}

#[test]
fn should_not_edit_unrelated_configuration_when_no_plugins_are_configured() {
    check(
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ]
}            
        "#,
        vec!["@semantic-release/changelog"],
        r#"
{
  "branches": [
    "master",
    "beta",
    "alpha"
  ]
}
        "#,
    )
}
