# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [v0.3.1] (unreleased)
- Put the tmdb hex into the config, instead of a secret variable
- Added CSP to the `_errorPages` and some other small fixes

### [v0.3.0] 

#### Added

- .yml files for automatic releases
- Modified rust comp updater to get the secrets using the environmental variables
- Added CSP to the pages

#### Changed

- Changed directories of the website
- Aesthetic changes and error handling improvements to the rust comp updater
- Fixed the top bar changing its width when typing on the comp page

#### Removed

- Removed Coming soon page
- Removed `Gamma-Boi`
- Removed inline javascript

### [v0.2.0]

#### Added

- Comp List
    - Added mobile support
    - Added comments to the javascript code

#### Changed

- General
    - Did **a lot of** cleanup and bug fixes with the rust updater script
- Main Page
    - Improved mobile support
    - Smoll styling changes
- Comp List
    - Check if the page search bar contains a number
    - fixed `minNumberElement` displaying that there is a page while there isn't
    - fixed `x results found` text to display the correct number
    - fixed css browser compatibility
    - removed unused css
    - Moved color accents to `required.css`
    - Made `updated_date` automatically change time format based on the user
- Updater Script
    - Fixed `game_skips.json` not getting saved
    - Made homebrew database updater into a separate function
    - Changed the last_updated string into `rfc3339` format
    - Tidied up the code for getting the `status_tag`, `code`, and `issue_type`
    - Improved error handling
    - Merged the two config classes into one
    - Fixed folder creation
    - Updated the config
    - Fixed direction of issues being flipped

### [v0.1.0]

#### Added

- General
    - added avif images (if supported on the client) for the logo's, this resulted in a ~90% size reduction of the
      images.
    - Added an animation for the header when it's done loading


- Main Page
    - Added an animation for the `main2` section.


- Comp List
    - Added an animation for the status bars when it's done fetching the `storage.json`
    - Added animations for the GameCard's
    - Added placeholders for when it's fetching the `storage.json`
    - Added id/code searching
    - Added page searching

#### Changed

- General
    - Improved text
    - Hid alt text on images
    - Finished `animationHandler()`


- Comp List
    - Changed the `Playable` color from <span style="background:#24bb2d;">#24bb2d</span>
      to <span style="background:#5fac4e;">#5fac4e</span>
    - Made `gameCardHandler()` only return the first 10 issues on load
    - Improved General code
    - Made the `N/A` image text for games without an image to `GAME`

#### Removed

- Comp List
    - Removed unnecessary id's for the status bar's
    - Removed `addEventListener` in favor of the `onInput` method for the search bar's