# Feature: All price/qty Are 006 Types

### Scope

- **Purpose**: One arithmetic substrate for every number in this workstream.
- **Responsibility**: Use workstream 006's types for every price and quantity, with no local numeric type.

### Statement

Every price and quantity anywhere in this workstream is a workstream-006 type — never a local numeric wrapper, and never a float — so hard problem 7's "no float" guarantee has exactly one place it could be violated.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:332` | Feature 24 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1055` | Feature 24's English title, Prompt 9 |
