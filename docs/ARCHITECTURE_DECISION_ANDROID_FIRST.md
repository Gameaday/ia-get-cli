# Architecture Decision: Android First-Party Support

## Decision Status: Proposed

**Date:** 2024
**Decision Makers:** Project Maintainers
**Status:** Awaiting approval

## Context and Problem Statement

The Internet Archive Helper project currently uses a Rust core with FFI bindings for mobile (Flutter) support. This approach has led to:

- Complex build pipelines integrating two different ecosystems (Cargo + Flutter)
- Race conditions and state synchronization issues between Rust and Dart
- Difficult debugging across language boundaries
- High maintenance burden for FFI bindings (14+ functions)
- Development friction requiring changes in both languages

**The Question:** How should we architect the project to make Android a true first-party platform while reducing complexity and maintaining code quality?

## Decision Drivers

### Technical Requirements
- ✅ Reliable mobile application without race conditions
- ✅ Maintainable codebase with clear architecture
- ✅ Fast development iteration cycle
- ✅ Good debugging and testing experience
- ✅ Performance suitable for mobile devices

### Business Requirements
- ✅ Deliver features faster
- ✅ Reduce technical debt
- ✅ Enable easier onboarding for contributors
- ✅ Maintain high code quality standards

### Non-Requirements
- ❌ Maximum code reuse at all costs
- ❌ Single codebase for all platforms
- ❌ Rust for everything

## Options Considered

### Option 1: Pure Flutter/Dart (Mobile-First) ✅ RECOMMENDED

**Description:** Rewrite mobile app entirely in Dart, keep CLI as separate Rust project.

**Architecture:**
```
┌─────────────────────────┐       ┌─────────────────────────┐
│   Rust CLI Project      │       │   Flutter Mobile App     │
│   (Desktop/Server)      │       │   (Android/iOS/Web)      │
│                         │       │                         │
│   • Pure Rust           │       │   • Pure Dart            │
│   • Command line focus  │       │   • Mobile-optimized     │
│   • High performance    │       │   • Rich UI              │
└─────────────────────────┘       └─────────────────────────┘
        ↓                                   ↓
   Power Users                         Mobile Users
```

**Pros:**
- ✅ Eliminates ALL FFI complexity
- ✅ No race conditions
- ✅ Single language per platform
- ✅ Fast development with hot reload
- ✅ Excellent debugging experience
- ✅ Rich Flutter ecosystem
- ✅ Mobile-first design philosophy
- ✅ Each platform optimized for its use case

**Cons:**
- ⚠️ Zero code sharing between CLI and mobile
- ⚠️ One-time rewrite cost (2-3 months)
- ⚠️ Separate maintenance for each platform
- ⚠️ Dart may be slower than Rust (though adequate for I/O)

**Effort:** High initial (rewrite), Low ongoing (easier maintenance)
**Risk:** Low (proven Flutter patterns)
**Code Reuse:** 0% (but that's the point!)

---

### Option 2: Simplified FFI Interface

**Description:** Keep FFI but drastically simplify to 3-5 functions only.

**Architecture:**
```
┌─────────────────────────────────────────┐
│        Flutter App (Primary)            │
│        • All state management           │
│        • Business logic in Dart         │
│        • UI and navigation              │
└─────────────────────────────────────────┘
                  ↓ Minimal FFI (3-5 functions)
┌─────────────────────────────────────────┐
│        Rust Library (Worker)            │
│        • Pure computation only          │
│        • No state                       │
│        • Heavy lifting functions        │
└─────────────────────────────────────────┘
```

**Pros:**
- ✅ High code reuse (80%)
- ✅ Simpler than current FFI
- ✅ Leverage Rust for performance
- ✅ State management unified in Dart

**Cons:**
- ⚠️ Still has FFI complexity (reduced, not eliminated)
- ⚠️ Still two build systems
- ⚠️ Still cross-language debugging
- ⚠️ Requires architecture redesign
- ⚠️ Race conditions still possible if not careful

**Effort:** Medium (redesign architecture)
**Risk:** Medium (FFI always has pitfalls)
**Code Reuse:** 80%

---

### Option 3: HTTP Bridge Architecture

**Description:** Run Rust as embedded HTTP service, Flutter connects via REST.

**Architecture:**
```
┌─────────────────────────┐
│   Flutter App           │
│   • Pure Dart UI        │
│   • HTTP client         │
└─────────────────────────┘
          ↓ HTTP/REST
┌─────────────────────────┐
│   Rust HTTP Service     │
│   • Local server        │
│   • Core logic          │
└─────────────────────────┘
```

**Pros:**
- ✅ Clean separation (HTTP boundary)
- ✅ High code reuse (90%)
- ✅ No FFI complexity
- ✅ Language agnostic
- ✅ Easy debugging (HTTP inspection)

**Cons:**
- ⚠️ HTTP overhead for local calls
- ⚠️ Process management complexity
- ⚠️ Battery impact from running service
- ⚠️ Still two runtimes
- ⚠️ More moving parts

**Effort:** Medium (implement HTTP layer)
**Risk:** Medium (process management issues)
**Code Reuse:** 90%

---

### Option 4: Keep Current Architecture

**Description:** Continue with existing Rust FFI + Flutter approach.

**Status:** This is what we're trying to move away from.

**Pros:**
- ✅ No migration needed
- ✅ High code reuse (85%)

**Cons:**
- ❌ Complex build system
- ❌ Race conditions
- ❌ Difficult debugging
- ❌ High maintenance burden
- ❌ Slow development velocity
- ❌ FFI binding overhead

**Effort:** Low (no change)
**Risk:** High (existing problems persist)
**Code Reuse:** 85%

## Decision Matrix

| Criteria | Pure Dart | Simplified FFI | HTTP Bridge | Current FFI |
|----------|-----------|---------------|-------------|-------------|
| **Complexity** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐ |
| **Maintainability** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐ |
| **Development Speed** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Debugging** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Code Reuse** | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Mobile-First** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Risk Level** | Low | Medium | Medium | High |
| **Initial Effort** | High | Medium | Medium | Low |
| **Long-term Value** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐ |

## Recommended Decision: Option 1 - Pure Flutter/Dart

### Rationale

**The Pure Dart approach is recommended because it:**

1. **Solves the Root Problem**
   - Eliminates ALL FFI complexity
   - No race conditions possible
   - Single source of truth (per platform)
   - Clear architectural boundaries

2. **Optimizes for the Right Thing**
   - Mobile users need great UX, not maximum code reuse
   - Desktop CLI users need performance, not mobile features
   - Each platform excels at what it does best

3. **Long-term Sustainability**
   - Easier to maintain two simple codebases than one complex integration
   - Standard patterns for each platform
   - Easier onboarding for contributors
   - Faster development velocity

4. **Proven Approach**
   - Many successful apps use platform-specific implementations
   - Flutter provides excellent mobile development experience
   - Rust provides excellent CLI experience
   - No need to force them together

### The "Code Reuse" Argument

**Traditional View:** "We should maximize code reuse between platforms!"

**Reality Check:** 
- Code reuse at 85% but complexity at 500% is a bad trade-off
- The cost of maintaining complex abstractions exceeds the cost of some duplication
- Platform-specific code allows platform-specific optimizations
- Shared algorithms can be documented rather than forced into shared code

**Better Metric:** Complexity per feature delivered

### Migration Strategy

#### Phase 1: Proof of Concept (3 weeks)
- Implement core functionality in pure Dart
- Build minimal viable app
- Compare complexity, performance, maintainability
- **Decision Point:** Go/No-Go based on results

#### Phase 2: Feature Implementation (6-8 weeks)
- Implement all core features
- Build comprehensive UI
- Add platform-specific optimizations
- Comprehensive testing

#### Phase 3: Transition (2-3 weeks)
- Release beta version
- Gather user feedback
- Fix issues and polish
- Official release

#### Phase 4: Long-term
- Maintain mobile app (Flutter/Dart)
- Maintain CLI tool (Rust)
- Share high-level patterns in documentation
- Accept different feature sets for different platforms

### Success Criteria

**Must Have:**
- ✅ No race conditions or FFI issues
- ✅ Feature parity with current mobile app
- ✅ Faster development iteration
- ✅ Better user experience
- ✅ Easier debugging

**Nice to Have:**
- ✅ Better performance than FFI version
- ✅ Smaller app size
- ✅ Lower battery consumption
- ✅ More platform-native feel

## Alternative Decisions

### If Pure Dart Proves Insufficient

**Fallback Option:** HTTP Bridge Architecture

If during the Proof of Concept phase (Phase 1), we discover that:
- Dart performance is inadequate
- Critical features are missing from Dart ecosystem
- Compression/decompression is too slow

Then we can pivot to the HTTP Bridge approach, which still eliminates FFI complexity while maintaining code reuse.

### If Resources Are Extremely Limited

**Minimal Approach:** Simplified FFI

If we absolutely cannot afford the rewrite, we should at minimum:
1. Reduce FFI functions from 14 to 3-5
2. Move ALL state management to Dart side
3. Use Rust ONLY for pure computation
4. Accept this as technical debt to be resolved later

## Consequences

### Positive Consequences
- ✅ Simpler, more maintainable codebase
- ✅ Faster feature development
- ✅ Better mobile user experience
- ✅ Each platform optimized for its use case
- ✅ Easier contributor onboarding

### Negative Consequences
- ⚠️ Initial rewrite effort (one-time cost)
- ⚠️ Two codebases to maintain
- ⚠️ Feature parity may drift between platforms
- ⚠️ Need expertise in both Rust and Dart

### Risk Mitigation
- Start with small proof of concept
- Measure results objectively
- Keep fallback options available
- Document decision and rationale

## Implementation Notes

### Technical Debt
- Current FFI implementation will be deprecated
- Mark FFI code as legacy
- Provide migration guide for any external users

### Documentation
- Update architecture diagrams
- Create migration guide
- Document decision rationale
- Update contributor guidelines

### Testing
- Maintain test coverage during migration
- Compare feature parity
- Performance benchmarks
- User acceptance testing

## Related Documents

- [FLUTTER_FIRST_ARCHITECTURE.md](FLUTTER_FIRST_ARCHITECTURE.md) - Detailed architectural exploration
- [PURE_DART_IMPLEMENTATION_GUIDE.md](PURE_DART_IMPLEMENTATION_GUIDE.md) - Implementation details
- [ANDROID_FEASIBILITY.md](ANDROID_FEASIBILITY.md) - Original FFI feasibility study

## Review and Updates

This decision should be reviewed:
- After proof of concept completion
- Every 6 months during implementation
- When significant new information emerges
- Before each major version release

---

**Next Steps:**
1. ✅ Create proof of concept (3 weeks)
2. ⏸️ Evaluate results and make final decision
3. ⏸️ If approved, begin full implementation
4. ⏸️ Maintain both versions during transition
5. ⏸️ Deprecate FFI implementation when ready

---

*This document follows the [Architecture Decision Records](https://adr.github.io/) pattern for documenting significant architectural decisions.*
