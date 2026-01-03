# Contributing to local-agent-core

Thank you for your interest in contributing to local-agent-core! This project aims to provide a privacy-first, local AI agent core, and we welcome contributions that align with this mission.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Maintain privacy-first principles in all contributions

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in Issues
2. Create a new issue with:
   - Clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Node version, etc.)

### Suggesting Features

1. Open an issue describing:
   - The feature and its use case
   - How it aligns with privacy-first principles
   - Potential implementation approach

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following our guidelines below
4. Write or update tests as needed
5. Ensure all tests pass (`npm test`)
6. Build the project (`npm run build`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to your branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Development Guidelines

### Code Style

- Follow TypeScript best practices
- Use meaningful variable and function names
- Add JSDoc comments for public APIs
- Run `npm run format` before committing
- Run `npm run lint` to check for issues

### Testing

- Write unit tests for all new functionality
- Maintain or improve code coverage
- Tests should be clear and focused
- Use descriptive test names

### Privacy & Security

- **Never** add features that send data externally without explicit user permission
- All data processing should be local by default
- Document any external dependencies clearly
- Security vulnerabilities should be reported privately

### Architecture Principles

1. **Modularity**: Keep components loosely coupled
2. **Extensibility**: Make it easy to add new LLM providers, action modules, etc.
3. **Privacy**: Local-first, permission-based, transparent
4. **Abstraction**: Hide implementation details behind clean interfaces
5. **Documentation**: Keep docs up-to-date with code changes

## Project Structure

```
src/
├── core/           # Core agent logic and types
├── llm/            # LLM provider interfaces and implementations
├── voice/          # Voice I/O abstractions
├── context/        # Context and habit tracking
├── actions/        # Action modules and permissions
├── utils/          # Utility functions
├── examples/       # Usage examples
└── __tests__/      # Unit tests
```

## Adding New Features

### Adding an LLM Provider

1. Extend `BaseLLMProvider` or implement `LLMProvider`
2. Implement `complete()` method
3. Add tests
4. Document usage in examples

### Adding an Action Module

1. Implement `ActionModule` interface
2. Define capabilities and risk levels
3. Add permission checks
4. Add tests
5. Create example usage

### Adding Context Features

1. Extend `ContextStore` if needed
2. Maintain privacy principles
3. Add tests for data retention
4. Document new context types

## Release Process

1. Version bump in package.json (semver)
2. Update CHANGELOG.md
3. All tests must pass
4. Documentation must be up-to-date
5. Create release notes

## Questions?

Feel free to open an issue for questions or discussions. We're here to help!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
