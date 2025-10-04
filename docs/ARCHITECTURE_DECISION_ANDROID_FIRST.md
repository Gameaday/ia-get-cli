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

### Option 1: Simplified FFI (Hybrid - Rust Core) ✅ **CO-RECOMMENDED**

**Description:** Keep Rust as core, dramatically simplify FFI, move all state to Dart.

**Architecture:**
```
┌─────────────────────────────────────────┐
│        Flutter App (State Owner)        │
│        • All state management           │
│        • Business logic in Dart         │
│        • UI and navigation              │
└─────────────────────────────────────────┘
          ↓ Minimal FFI (5 functions only)
┌─────────────────────────────────────────┐
│     Rust Core (Stateless Engine)        │
│     • Pure computation functions        │
│     • No state management               │
│     • Performance-critical operations   │
└─────────────────────────────────────────┘
```

**Pros:**
- ✅ Keeps Rust as single source of truth
- ✅ 64% reduction in FFI complexity (14 → 5 functions)
- ✅ Eliminates race conditions (state in Dart only)
- ✅ Maintains 90%+ code reuse
- ✅ Supports platforms Flutter doesn't run on
- ✅ Clear separation: Rust=computation, Dart=state
- ✅ CLI remains fully independent
- ✅ Best of both worlds

**Cons:**
- ⚠️ Still has FFI (but much simpler)
- ⚠️ Still two build systems
- ⚠️ Requires architectural redesign (2 months)

**Effort:** Medium (redesign architecture)
**Risk:** Low (proven pattern, simpler than current)
**Code Reuse:** 90%+

---

### Option 2: Pure Flutter/Dart (Mobile-First) ✅ **CO-RECOMMENDED**

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

### Option 3: Simplified FFI Interface

**Description:** Keep FFI but drastically simplify to 3-5 functions only.

*Note: This is essentially the same as Option 1 above. See Option 1 for full details.*

---

### Option 4: HTTP Bridge Architecture

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

### Option 5: Keep Current Architecture

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

| Criteria | Simplified FFI | Pure Dart | HTTP Bridge | Current FFI |
|----------|---------------|-----------|-------------|-------------|
| **Complexity** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ |
| **Maintainability** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ |
| **Development Speed** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Debugging** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Code Reuse** | ⭐⭐⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Mobile-First** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Platform Support** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Rust Core** | ⭐⭐⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Risk Level** | Low | Low | Medium | High |
| **Initial Effort** | Medium | High | Medium | Low |
| **Long-term Value** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ |

## Recommended Decision: Two Co-Recommended Options

Based on the analysis, there are **two equally valid approaches** depending on your priorities:

### Option A: Simplified FFI (Hybrid) - **If Keeping Rust is Priority**

**Choose this if:**
- ✅ You want Rust to remain the heart of the project
- ✅ You need support for platforms Flutter doesn't run on (embedded Linux, FreeBSD, servers)
- ✅ You value maximum code reuse (90%+)
- ✅ You want native performance for all operations
- ✅ You're willing to maintain FFI (albeit much simpler)

**Result:** Rust stays as core, FFI complexity reduced by 64%, race conditions eliminated.

**See:** [RUST_CORE_FLUTTER_INTEGRATION.md](RUST_CORE_FLUTTER_INTEGRATION.md) for detailed implementation.

### Option B: Pure Flutter/Dart - **If Mobile-First is Priority**

**Choose this if:**
- ✅ Mobile is the primary platform
- ✅ You want the absolute simplest architecture
- ✅ You're willing to accept separate CLI and mobile codebases
- ✅ You prioritize development velocity over code reuse
- ✅ You want zero FFI complexity

**Result:** Two simple, independent codebases optimized for their platforms.

## Recommendation Based on Your Comment

Given your statement: *"I don't really feel like killing rust out of the project is what I want to do because it feels like the heart of where I started"* and the need for non-Flutter platforms:

**→ Simplified FFI (Option A) is recommended for your use case.**

### Rationale for Simplified FFI Approach

**The Simplified FFI approach is recommended when keeping Rust is a priority because it:**

1. **Preserves Rust as the Heart**
   - Rust remains the core computation engine
   - All platforms use the same Rust logic
   - CLI maintains full independence
   - Can run on any Rust-supported platform

2. **Dramatically Reduces Complexity**
   - 64% reduction in FFI functions (14 → 5)
   - Eliminates all race conditions
   - State management unified in Dart
   - Clear architectural boundaries

3. **Best of Both Worlds**
   - Rust performance and platform reach
   - Flutter UI excellence and developer experience
   - Simple FFI without current complexity
   - High code reuse (90%+)

4. **Platform Flexibility**
   - Rust core runs on: Android, iOS, Windows, Mac, Linux, embedded Linux, FreeBSD, servers
   - Flutter runs on: Android, iOS, Web, Desktop
   - CLI works everywhere Rust runs

### The Key Insight

**You don't need to kill Rust to fix the FFI complexity!**

The problem isn't Rust or FFI itself—it's **where the state lives**. Current architecture has state in both Rust and Dart, causing race conditions.

**Solution:** Move ALL state to Dart, make Rust stateless. This gives you:
- Simple FFI (5 functions vs 14)
- No race conditions (state in one place)
- Rust remains the source of truth for operations
- Flutter gets full control over state and UI

### Alternative: Pure Dart Approach

For context, the Pure Dart approach has these characteristics:

1. **Solves Complexity Completely**
   - Eliminates ALL FFI complexity
   - No race conditions possible
   - Simplest architecture

2. **Trade-offs**
   - 0% code sharing between platforms
   - Mobile-only focus (doesn't help CLI/server use cases)
   - Dart performance adequate but not as fast as Rust

3. **Best For**
   - Projects where mobile is the only priority
   - Teams that want zero FFI maintenance
   - Cases where code reuse isn't important

**Given your priorities (keeping Rust as core, supporting non-Flutter platforms), the Simplified FFI approach is better.**

### Migration Strategy for Simplified FFI

#### Phase 1: Redesign Rust Core (2-3 weeks)
- [ ] Separate core logic from state management
- [ ] Create stateless function versions
- [ ] Add synchronous wrappers for FFI
- [ ] Test independently

#### Phase 2: Simplify FFI Layer (1-2 weeks)
- [ ] Reduce to 5 core functions
- [ ] Remove state from FFI
- [ ] Simplify error handling
- [ ] Update bindings

#### Phase 3: Update Flutter (2-3 weeks)
- [ ] Move all state to Dart
- [ ] Update FFI bindings
- [ ] Use Isolates for blocking calls
- [ ] Test thoroughly

#### Phase 4: Deprecate Old FFI (1 week)
- [ ] Mark old functions deprecated
- [ ] Update documentation
- [ ] Provide migration guide

**Total: 6-9 weeks (1.5-2 months)**

### Implementation Comparison

**Current FFI (Complex):**
```rust
// 14 functions managing state
ia_get_init()
ia_get_create_session()
ia_get_pause_download()
ia_get_resume_download()
ia_get_get_session_info()
// ... 9 more
```

**Simplified FFI (Clean):**
```rust
// 5 stateless functions
ia_get_fetch_metadata(id) -> json
ia_get_download_file(url, path, callback) -> result
ia_get_decompress_file(path, output) -> files
ia_get_validate_checksum(file, hash) -> bool
ia_get_last_error() -> string
```

### Migration Strategy (Alternative: Pure Dart)

If you later decide Pure Dart is better:

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
