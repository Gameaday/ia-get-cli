# Integration Testing Checklist - Phase 4 Task 2

**Feature**: Advanced Search & Filters  
**Date**: October 7, 2025  
**Status**: Ready for Testing

## Prerequisites

- [ ] CI build passed successfully
- [ ] App installed on test device (Android/iOS)
- [ ] Test both light and dark mode
- [ ] Test both portrait and landscape orientations
- [ ] Clear app data before starting comprehensive tests

## Test Environment Setup

### Devices to Test
- [ ] Android phone (portrait)
- [ ] Android tablet (landscape)
- [ ] iOS phone (if available)
- [ ] Android emulator with varying screen sizes

### Theme Testing
- [ ] Light mode
- [ ] Dark mode
- [ ] System theme (auto-switching)

---

## 1. Navigation Flow Testing

### HomeScreen → AdvancedSearchScreen
- [ ] Tap "Advanced Search" icon (manage_search) in HomeScreen app bar
- [ ] Screen transitions with MD3 sharedAxis animation
- [ ] AdvancedSearchScreen loads without errors
- [ ] Back button returns to HomeScreen correctly

### AdvancedSearchScreen → SavedSearchesScreen
- [ ] Tap bookmark icon in app bar
- [ ] SavedSearchesScreen opens with MD3 transition
- [ ] Tap "View All" button in saved searches section
- [ ] Both paths lead to same screen
- [ ] Back button returns to AdvancedSearchScreen

### SavedSearchesScreen → AdvancedSearchScreen (with data)
- [ ] Select a saved search from list
- [ ] Returns to AdvancedSearchScreen with loaded search
- [ ] All fields populated correctly (query, filters, date range, sort)
- [ ] Field-specific search section shown if needed

### AdvancedSearchScreen → SearchResultsScreen
- [ ] Enter search query and tap Search FAB
- [ ] SearchResultsScreen opens with MD3 fadeThrough transition
- [ ] Query string shown in app bar title
- [ ] Back button returns to AdvancedSearchScreen

### SearchResultsScreen → ArchiveDetailScreen
- [ ] Tap on a search result card
- [ ] Loading indicator shows while fetching metadata
- [ ] ArchiveDetailScreen opens with MD3 fadeThrough transition
- [ ] Archive details loaded correctly
- [ ] Back button returns to SearchResultsScreen

---

## 2. AdvancedSearchScreen Features

### Search Field
- [ ] Type in main search field
- [ ] Autocomplete suggestions appear
- [ ] Suggestions populated from search history
- [ ] Tap suggestion to fill search field
- [ ] Clear button (X) clears search field
- [ ] Search field hint text displays correctly

### Field-Specific Search
- [ ] Tap "Field-Specific Search" to expand section
- [ ] Title field input works
- [ ] Creator field input works
- [ ] Subject field input works
- [ ] All fields persist when toggling section
- [ ] Collapse button hides section

### Mediatype Filters
- [ ] All mediatype chips displayed (Texts, Movies, Audio, Image, Software, Data, Web)
- [ ] Single-select: selecting one deselects others
- [ ] Each mediatype chip shows correct icon
- [ ] Selected chip has proper MD3 styling
- [ ] Selection persists during session

### Date Range Filtering

#### Preset Ranges
- [ ] "Last Week" preset works
- [ ] "Last Month" preset works
- [ ] "Last Year" preset works
- [ ] "Last 5 Years" preset works
- [ ] Selected preset shows in UI
- [ ] Clear date range button works

#### Custom Date Range
- [ ] Tap "Custom Range" button
- [ ] Custom date picker dialog opens
- [ ] Select start date
- [ ] Select end date (validates after start date)
- [ ] Duration in days displays correctly
- [ ] Cancel button dismisses without applying
- [ ] Apply button (enabled only when both dates selected)
- [ ] Custom range applied and shown in snackbar
- [ ] Clear button removes custom range

### Sort Options
- [ ] All sort options displayed (Relevance, Date, Title, Downloads, Views)
- [ ] Selecting sort option updates UI
- [ ] Only one sort option selected at a time
- [ ] Selected sort has proper MD3 chip styling

### Search History
- [ ] Recent searches section displays
- [ ] Shows last 5 searches
- [ ] Each search shows query and timestamp
- [ ] Tap search to fill main field
- [ ] Searches older than 90 days auto-removed

### Saved Searches Preview
- [ ] Shows first 3 saved searches
- [ ] Pinned searches shown first
- [ ] Each card shows name, query, and timestamp
- [ ] Tap saved search loads into form
- [ ] "View All" navigates to SavedSearchesScreen

### Actions
- [ ] Save Search button works
- [ ] Dialog prompts for name and description
- [ ] Saved search appears in list
- [ ] Clear filters button resets all filters
- [ ] Search FAB only enabled when query entered
- [ ] Search FAB animates on scroll

---

## 3. SavedSearchesScreen Features

### Display & Sorting
- [ ] All saved searches displayed
- [ ] Pinned searches shown at top
- [ ] Sort by: Last Used works
- [ ] Sort by: Name (A-Z) works
- [ ] Sort by: Created Date works
- [ ] Empty state shown when no searches

### Search Within Saved
- [ ] Search bar filters saved searches
- [ ] Filters by name and query
- [ ] Clear search button (X) works
- [ ] Results update in real-time

### Tag Management
- [ ] All unique tags shown as filter chips
- [ ] Select tag to filter
- [ ] Multiple tags can be selected
- [ ] Clear tag filters works
- [ ] Tag filtering combines with text search

### Saved Search Actions
- [ ] Tap search card loads into AdvancedSearchScreen
- [ ] Pin/Unpin icon toggles correctly
- [ ] Pin icon changes (push_pin vs bookmark_outline)
- [ ] Edit action opens edit dialog
- [ ] Edit dialog shows current name/description
- [ ] Save edit updates search
- [ ] Delete action shows confirmation
- [ ] Delete removes search
- [ ] Usage count increments when loaded

### Edit Dialog
- [ ] Name field pre-filled
- [ ] Description field pre-filled (if exists)
- [ ] Tags input works
- [ ] Comma-separated tags parsed correctly
- [ ] Cancel dismisses without changes
- [ ] Save applies changes

---

## 4. SearchResultsScreen Features

### Search Execution
- [ ] Loading state shows "Searching Internet Archive..."
- [ ] Results load and display
- [ ] Result count shown in app bar
- [ ] Empty state shown for no results
- [ ] Error state shown for search errors
- [ ] Retry button re-executes search

### Results Display
- [ ] Each result card shows title, description, identifier
- [ ] Card layout follows MD3 guidelines
- [ ] Title truncates at 2 lines
- [ ] Description truncates at 3 lines
- [ ] Identifier truncates at 1 line
- [ ] Chevron icon indicates tap action

### Pagination
- [ ] Initial page loads (20 results)
- [ ] Scroll near bottom triggers load more
- [ ] Loading indicator shows while loading more
- [ ] New results append to list
- [ ] Pagination stops when no more results
- [ ] Error message shown if load more fails

### Pull-to-Refresh
- [ ] Pull down to trigger refresh
- [ ] Loading indicator shows
- [ ] Results refresh from first page
- [ ] Error handling works on refresh

### Navigation to Detail
- [ ] Tap result card
- [ ] Loading occurs (metadata fetch)
- [ ] ArchiveDetailScreen opens
- [ ] Correct archive loaded
- [ ] Error shown if metadata fails

---

## 5. Material Design 3 Compliance

### Colors & Theming
- [ ] All colors from theme palette (no hardcoded)
- [ ] Proper contrast in light mode (WCAG AA+)
- [ ] Proper contrast in dark mode (WCAG AA+)
- [ ] Surface colors appropriate (surface, surfaceVariant)
- [ ] Outline colors used correctly

### Typography
- [ ] All text uses theme text styles
- [ ] titleLarge, titleMedium, titleSmall used correctly
- [ ] bodyLarge, bodyMedium, bodySmall used correctly
- [ ] labelLarge, labelMedium, labelSmall used correctly
- [ ] Font scaling works (0.8x to 1.2x)

### Spacing & Layout
- [ ] 4dp grid system followed
- [ ] Padding: 16dp, 24dp used consistently
- [ ] Spacing: 8dp, 12dp, 16dp between elements
- [ ] Cards have 12dp margin
- [ ] Touch targets minimum 48dp

### Components
- [ ] FAB follows MD3 spec (56dp, correct elevation)
- [ ] Chips follow MD3 spec (outlined/filled)
- [ ] Cards follow MD3 spec (elevation 1)
- [ ] Dialogs follow MD3 spec
- [ ] Buttons follow MD3 spec (Filled, Tonal, Text)
- [ ] App bars follow MD3 spec

### Animations
- [ ] Page transitions use MD3 curves
- [ ] fadeThrough: emphasized curve, medium duration
- [ ] sharedAxis: emphasized curve, medium duration
- [ ] Component animations smooth (300-500ms)
- [ ] No jank or stuttering

### Elevation & Shadows
- [ ] Cards: elevation 1 (subtle shadow)
- [ ] FAB: elevation 3 (clear shadow)
- [ ] Dialogs: elevation 5 (prominent shadow)
- [ ] No conflicting elevations

---

## 6. Service Integration Testing

### SearchHistoryService
- [ ] Entries saved to database
- [ ] Autocomplete retrieves recent entries
- [ ] Cleanup removes entries >90 days
- [ ] Popular searches calculated correctly
- [ ] Service persists across app restarts

### SavedSearchService
- [ ] Searches saved to database
- [ ] Pin/unpin persists
- [ ] Tags saved and retrieved
- [ ] Usage count increments
- [ ] Edit updates correctly
- [ ] Delete removes from database
- [ ] Service persists across app restarts

### AdvancedSearchService
- [ ] Search executes against Internet Archive API
- [ ] Query string built correctly
- [ ] Pagination works (page parameter)
- [ ] Results parsed correctly
- [ ] Error handling for network failures
- [ ] Error handling for API errors
- [ ] Timeout handling (30 seconds)

---

## 7. Performance Testing

### Load Times
- [ ] AdvancedSearchScreen loads < 500ms
- [ ] SavedSearchesScreen loads < 300ms
- [ ] SearchResultsScreen first paint < 200ms
- [ ] Search execution < 3 seconds (network dependent)
- [ ] Pagination load more < 2 seconds

### Memory Usage
- [ ] No memory leaks on navigation
- [ ] Scroll performance smooth (60 FPS)
- [ ] Large result lists don't cause lag
- [ ] Image loading doesn't block UI

### Battery Impact
- [ ] No excessive background activity
- [ ] Network requests batched appropriately
- [ ] Animations GPU-accelerated

---

## 8. Edge Cases & Error Handling

### Network Errors
- [ ] No internet: Shows error with retry
- [ ] Timeout: Shows timeout error
- [ ] API error: Shows error message
- [ ] Malformed response: Graceful degradation

### Data Edge Cases
- [ ] Empty search query: FAB disabled
- [ ] No saved searches: Empty state shown
- [ ] No search results: Empty state shown
- [ ] Very long titles: Truncated properly
- [ ] Special characters in query: Handled correctly
- [ ] Unicode characters: Displayed correctly

### State Management
- [ ] Back navigation clears state correctly
- [ ] App backgrounding preserves state
- [ ] App foregrounding restores state
- [ ] Screen rotation preserves state
- [ ] Process death restores critical state

### Boundary Conditions
- [ ] Maximum query length (if any)
- [ ] Maximum saved searches (database limit)
- [ ] Maximum search history entries (90 days)
- [ ] Date range edge cases (same day, future dates)
- [ ] Empty filter selections

---

## 9. Accessibility Testing

### Screen Reader (TalkBack)
- [ ] All buttons have semantic labels
- [ ] All images have descriptions
- [ ] Navigation order logical
- [ ] Announcements clear and helpful
- [ ] Form fields properly labeled

### Touch Targets
- [ ] All interactive elements ≥ 48dp
- [ ] Chips large enough to tap
- [ ] List items easy to select
- [ ] FAB easy to reach

### Text & Contrast
- [ ] Text readable at system font sizes
- [ ] 1.2x font scaling works
- [ ] All text meets WCAG AA contrast
- [ ] Focus indicators visible

---

## 10. Platform-Specific Testing

### Android
- [ ] Back button navigation works
- [ ] Hardware back button handled
- [ ] Share intent works (if applicable)
- [ ] Deep links work (if applicable)
- [ ] Notifications work (if applicable)

### iOS (if available)
- [ ] Swipe back gesture works
- [ ] iOS share sheet works
- [ ] Universal links work (if applicable)
- [ ] iOS notifications work (if applicable)

---

## Issue Tracking Template

When bugs are found, document with:

**Issue**: Brief description  
**Screen**: Which screen/feature  
**Steps to Reproduce**:
1. Step 1
2. Step 2
3. Step 3

**Expected**: What should happen  
**Actual**: What actually happens  
**Severity**: Critical / High / Medium / Low  
**Screenshots**: Attach if helpful

---

## Sign-Off

### Testing Completed By
- **Tester**: _________________
- **Date**: _________________
- **Build Version**: _________________
- **Device(s)**: _________________

### Results Summary
- **Total Tests**: _____
- **Passed**: _____
- **Failed**: _____
- **Blocked**: _____
- **Pass Rate**: _____%

### Critical Issues Found
1. _________________
2. _________________
3. _________________

### Recommendation
- [ ] ✅ **Approved for Production** - All critical tests passed
- [ ] ⚠️ **Approved with Minor Issues** - Non-critical issues documented
- [ ] ❌ **Not Approved** - Critical issues must be fixed

---

## Notes

Use this checklist systematically. Test one section at a time and mark each item as you go. Don't rush—thorough testing now prevents production issues later.

For any failures, create detailed bug reports and track them until resolved.
