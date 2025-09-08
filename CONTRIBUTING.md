# Contributing to Botanica

Thanks for your interest in contributing to the botanical database infrastructure!

## How to Contribute
1. Fork the repo and create a branch (`git checkout -b feature/botanical-enhancement`).
2. Make your changes with clear commits and comprehensive tests.
3. Run the full test suite to ensure nothing breaks (`cargo test --all-features`).
4. Open a Pull Request against `main` with detailed description.

## Code Style
- Rust 2021 edition with `cargo fmt` and `cargo clippy` before submitting.
- Comprehensive error handling using `Result<T, BotanicalError>`.
- All public APIs must have documentation with examples.
- New features require corresponding test coverage.

## Contribution Scope
Features should align with the **Botanica philosophy**:  
- **Scientific accuracy**: Proper taxonomic nomenclature and botanical standards
- **Production ready**: Memory-safe, well-tested, comprehensive error handling  
- **Invisible infrastructure**: Simple APIs that botanical applications just work with
- **Free forever**: No features that could lead to paid tiers or restrictions

## Review Process
- All PRs require review and approval from the lead maintainer.
- Botanical accuracy will be verified against scientific literature.
- Performance impact will be measured for database operations.
- Merge authority is reserved to maintain project direction and quality.

## Testing Requirements
- Unit tests for all new functions and data structures.
- Integration tests for database operations and API endpoints.
- Performance benchmarks for query operations affecting large datasets.
- Documentation tests to ensure examples remain current.

## Botanical Standards
- Follow APG IV classification system for plant families.
- Use proper authority citations (e.g., "Rosaceae Juss.", "Rosa L.").
- Include publication years for species descriptions when available.
- Follow IUCN Red List categories for conservation status.

## Recognition
Contributors are acknowledged in `AUTHORS.md` after a merged PR.
Significant contributors may be invited to co-maintain specific modules.