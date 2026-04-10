# Link Test Document

## Internal Anchors

- [Valid link to Title](#link-test-document) - should pass
- [Valid link to Internal Anchors](#internal-anchors) - should pass
- [Broken anchor](#nonexistent-section) - should fail
- [Another broken](#does-not-exist) - should fail

## Relative Links

- [Existing file](./trailing_whitespace.md) - should pass
- [Missing file](./does_not_exist.md) - should fail

## External Links

- [Example](https://example.com) - valid external
- [Google](https://google.com) - valid external
