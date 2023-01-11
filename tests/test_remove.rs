use std::collections::HashSet;
use std::iter::FromIterator;

use configure_semantic_release_manifest::SemanticReleaseManifest;

fn check(initial: &str, to_remove: Vec<&str>, expected: &str) {
    let mut manifest = SemanticReleaseManifest::parse_from_string(initial).unwrap();
    manifest.remove_plugin_configuration(HashSet::from_iter(
        to_remove.into_iter().map(|s| s.to_owned()),
    ));
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
