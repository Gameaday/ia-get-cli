# Test Cases for Type Conversion Fixes

This document outlines the test cases that should be run to verify the type conversion fixes in the Flutter mobile app.

## Issue Description

The Internet Archive API sometimes returns numeric fields (like `size` and `item_size`) as Strings instead of integers. This caused runtime errors like:

```
type 'String' is not a subtype of type 'int'
```

## Solution

Added `_parseIntField` helper methods to both `ArchiveMetadata` and `ArchiveFile` classes that handle both String and int types gracefully.

## Test Files

Due to the project's .gitignore configuration, unit test files in `mobile/flutter/test/` are not tracked. However, comprehensive test files have been created:

- `mobile/flutter/test/models/archive_metadata_test.dart` - Tests for ArchiveMetadata and ArchiveFile type conversions
- `mobile/flutter/test/services/internet_archive_api_test.dart` - Tests for API error handling and identifier suggestions

These files contain tests for:

### Type Conversion Tests (archive_metadata_test.dart)

1. **Size field parsing as int when provided as int**
   - Verifies that native int values are handled correctly

2. **Size field parsing as int when provided as String**
   - Verifies that String values like "1024" are converted to int

3. **Handling null size values**
   - Ensures null values are handled gracefully

4. **Handling empty String size values**
   - Ensures empty strings are converted to null

5. **Handling invalid String size values**
   - Ensures non-numeric strings are converted to null without throwing

6. **Parsing large size values**
   - Tests with values like "5368709120" (5GB) as String

7. **item_size field parsing in ArchiveMetadata**
   - Same tests as above but for the totalSize field

8. **Complete metadata parsing with mixed types**
   - Tests a realistic scenario with some fields as String and others as int

### Identifier Suggestion Tests (internet_archive_api_test.dart)

1. **Suggesting lowercase version when uppercase identifier not found**
   - When "TestItem" fails, suggests "testitem" if it exists

2. **Suggesting similar identifiers from search results**
   - When identifier not found, searches for similar items and suggests them

3. **Returning basic not found message when no suggestions available**
   - Falls back to simple error message if no suggestions can be found

4. **Handling empty search results gracefully**
   - Doesn't crash when search API returns no results

5. **Prioritizing lowercase suggestion over search results**
   - When both exist, lowercase suggestion is shown first

6. **Handling rate limiting with retry-after**
   - Tests that the API respects the Retry-After header

7. **Handling server errors with exponential backoff**
   - Tests automatic retry logic for 5xx errors

8. **Throwing exception after max retries**
   - Ensures the app doesn't retry indefinitely

## Running the Tests

To run these tests (once they're not ignored by .gitignore):

```bash
cd mobile/flutter
flutter test test/models/archive_metadata_test.dart
flutter test test/services/internet_archive_api_test.dart
```

Or run all tests:

```bash
cd mobile/flutter
flutter test
```

## Manual Testing Scenarios

If automated tests cannot be run, manually verify:

1. **Test with an archive that has String size values**
   - Search for and load an archive item
   - Verify no type errors occur
   - Check that file sizes display correctly

2. **Test with uppercase identifier**
   - Try to load an archive like "CommuteTEST"
   - Verify suggestion to try "commutetest" appears

3. **Test with invalid identifier**
   - Try to load a non-existent archive
   - Verify suggestions for similar items appear

4. **Test with mixed case identifier**
   - Try "MixedCase" when "mixedcase" exists
   - Verify case-sensitivity message appears

## Expected Results

All tests should pass, verifying that:

1. The app handles both String and int types for numeric fields
2. The app suggests alternatives when an identifier is not found
3. The app handles API errors gracefully with retries
4. The app provides helpful error messages to users

## API Compliance

The solution maintains full compliance with the Internet Archive API by:

- Accepting both String and int types as documented in the API
- Handling all documented HTTP status codes (404, 429, 5xx)
- Respecting rate limits and Retry-After headers
- Providing helpful error messages that guide users to correct identifiers
