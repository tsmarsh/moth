# Severity Is Configurable

Allow users to define custom severity levels in config instead of hardcoded `crit/high/med/low`. Support unicode/emoji.

## Design
- Simple list of strings in config
- Order in list = sort priority (first = highest)
- Permissive loading: unknown severities accepted
- Default: `[crit, high, med, low]` when absent

## Config Format
```yaml
severities:
  - crit
  - high
  - med
  - low
default_severity: med
```

## Implementation
1. **config.rs**: Add `severities: Vec<String>`, `get_severity_rank()` helper
2. **issue.rs**: Remove `Severity` enum, use `String`
3. **store.rs**: Config-based sorting, validation in `create_issue()`
4. **Commands**: Pass strings, validate against config
5. **Tests**: Update to string comparisons
