# Version 0.6.11 (2024-10-22)

- Added -D flag for dependency generation. JSON output format

# Version 0.6.10 (2024-10-15)

- Added support for skill chat codes

# Version 0.6.9 (2024-05-18)

- Removed build.rs to remove dependency on gw2search

# Version 0.6.8 (2024-05-17)

- Added build.rs script to auto-download data files from api
- Added elite true/false flag to specialization offline data (unused)

# Version 0.6.7 (2024-04-02)

- Added legends to offline data source
- Improved some error handling

# Version 0.6.5 (2024-04-01)

- Converted specializations, professions and pets to use hardcoded json data instead of hitting api (legends still pending)

# Version 0.6.3 (2024-03-21)

- Added aw2/armory markup for pet icons via the api (needs css cleanup)

# Version 0.6.2 (2024-03-20)

- Added library support for bidirectional chatlink conversion
- Initial automated release

## v0.6.11 (2025-07-04)

### New Features

 - <csr-id-ae97051664bfeab2b78d508db5bdc05960ac9909/> skill palette bit size was too small. Increased to 32

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 262 calendar days.
 - 262 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr v0.6.11 ([`a681fc9`](https://github.com/berdandy/chatr/commit/a681fc939df84c04dfd44be8c9b8508a85120dcb))
    - Skill palette bit size was too small. Increased to 32 ([`ae97051`](https://github.com/berdandy/chatr/commit/ae97051664bfeab2b78d508db5bdc05960ac9909))
    - Added preliminary build notes. Mostly memory things ([`c9e02e6`](https://github.com/berdandy/chatr/commit/c9e02e61f4b4bd4a4eaac6526e7d8784eb7a62da))
    - Removed change tracking. This doesn't really fit the scope of chatr, so will be in a new lib specifically for balance patches called fulcrum ([`c056563`](https://github.com/berdandy/chatr/commit/c0565630e54a4770fa3d1b2ca46af40305d21d85))
    - Added a preliminary GearTemplate ([`24fe641`](https://github.com/berdandy/chatr/commit/24fe641458dde89e039118f6b01c96cafba1cae2))
    - Added preliminary code for gear templates ([`323f518`](https://github.com/berdandy/chatr/commit/323f5189ef2818ee04aa3ffc6791277c0cbffdc6))
    - Some preliminary work on an update check mechanism ([`37bdc33`](https://github.com/berdandy/chatr/commit/37bdc3371c1292b1bbfadc3fd1b124094c8fa159))
    - Component version update ([`89c98f0`](https://github.com/berdandy/chatr/commit/89c98f0faca9c5cf06c7d12bf2578252ba3d5df8))
    - Bump openssl from 0.10.70 to 0.10.72 ([`094d1d6`](https://github.com/berdandy/chatr/commit/094d1d6f04a66f63fd8758056f93251c9214a957))
    - Bump openssl from 0.10.68 to 0.10.70 ([`6bb54a0`](https://github.com/berdandy/chatr/commit/6bb54a0a64cb42af50168822ef65169865918119))
    - Fixed example code ([`f74d354`](https://github.com/berdandy/chatr/commit/f74d3544034caf75f22e53e684a2ed7fbd44dcef))
    - Correct json for dependency generation ([`76bf86d`](https://github.com/berdandy/chatr/commit/76bf86df694162f530f768f573b49a62c25dd064))
    - Added -D dependency generation flag ([`e9e9d96`](https://github.com/berdandy/chatr/commit/e9e9d96c688201a00595e3baef00f7f291dd839d))
    - Added Code trait ([`271392a`](https://github.com/berdandy/chatr/commit/271392ad6ce2bb9597744e07792de373dc341a0f))
</details>

## v0.6.10 (2024-10-15)

<csr-id-64fea68e878b120b3a21ae5b0d579217a5f6dd0f/>

### Chore

 - <csr-id-64fea68e878b120b3a21ae5b0d579217a5f6dd0f/> Release chatr version 0.6.10

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 80 calendar days.
 - 150 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.10 ([`64fea68`](https://github.com/berdandy/chatr/commit/64fea68e878b120b3a21ae5b0d579217a5f6dd0f))
    - Added changelog for 0.6.10 (skill codes) ([`03bdc52`](https://github.com/berdandy/chatr/commit/03bdc520b0b675432d45b37d78e3052b4fb2e5ef))
    - Added Skill chat links to chat code processing ([`11cd39d`](https://github.com/berdandy/chatr/commit/11cd39d16a4cd2ee44537a6b9f91404e411b1355))
    - Adding skill template and using deku magic ([`07c8a6d`](https://github.com/berdandy/chatr/commit/07c8a6d82a90c73460484ea6f9d494e5b14f4957))
    - Bump openssl from 0.10.62 to 0.10.66 ([`39098b1`](https://github.com/berdandy/chatr/commit/39098b164c4f138e6e39c76cd19a21dde2d2f367))
</details>

## v0.6.9 (2024-05-18)

<csr-id-7805262da403aff199f5f33208139eed5844ebef/>

### Chore

 - <csr-id-7805262da403aff199f5f33208139eed5844ebef/> Release chatr version 0.6.9

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.9 ([`7805262`](https://github.com/berdandy/chatr/commit/7805262da403aff199f5f33208139eed5844ebef))
    - Changed my mind. While build.rs is interesting, I would have to ship (or include) gw2search. Just gitadding artifacts instead ([`16f1e1d`](https://github.com/berdandy/chatr/commit/16f1e1d4c0cfdee2594161e9fb1fa836e4a9c93d))
    - Only build json if build.rs changed ([`389663c`](https://github.com/berdandy/chatr/commit/389663c56c85986d47e5f3000c765ead76fd2e22))
</details>

## v0.6.8 (2024-05-17)

<csr-id-2af09eca0e3aaf9a46334c94af87665f9258849a/>

### Chore

 - <csr-id-2af09eca0e3aaf9a46334c94af87665f9258849a/> Release chatr version 0.6.8

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 39 calendar days.
 - 45 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.8 ([`2af09ec`](https://github.com/berdandy/chatr/commit/2af09eca0e3aaf9a46334c94af87665f9258849a))
    - Changelog update ([`2b89e99`](https://github.com/berdandy/chatr/commit/2b89e99645846fb1c0d07ec62a0607cfeca446be))
    - Added elite data into specialization json ([`066ae50`](https://github.com/berdandy/chatr/commit/066ae5029eb86830f1f5ebbf0c69d2bfc0a63738))
    - Added build script to download json files ([`ef3dc7b`](https://github.com/berdandy/chatr/commit/ef3dc7ba51c4d3f9ead85a3be6264c4e6296c16d))
    - Bump h2 from 0.3.24 to 0.3.26 ([`e94f6f4`](https://github.com/berdandy/chatr/commit/e94f6f49e9547806e43be4646fef50b41dc3628b))
    - Clippy happy joy fun time ([`6d5d367`](https://github.com/berdandy/chatr/commit/6d5d367544c781eacf0921ea615803d8c0315b5f))
</details>

## v0.6.7 (2024-04-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improved error reporting ([`43fb636`](https://github.com/berdandy/chatr/commit/43fb6369191ac123ed9468939e62d90dc451134f))
</details>

## v0.6.6 (2024-04-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added legends offline data ([`b29c33b`](https://github.com/berdandy/chatr/commit/b29c33bb9d62492eb3749abdcedee5d9eff877d6))
</details>

## v0.6.5 (2024-04-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 5 calendar days.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release version bump ([`566c8ea`](https://github.com/berdandy/chatr/commit/566c8ea04f1a622920a2a7bb1484c4a248a528c3))
    - Partial offline conversion ([`2e17be5`](https://github.com/berdandy/chatr/commit/2e17be58c726e28b3667b5fbdb6517a0d3d80b47))
    - Changed latest to know datestamp ([`b7da3c0`](https://github.com/berdandy/chatr/commit/b7da3c0d24e9889dde7eff5eb183a5627a57e22e))
</details>

## v0.6.4 (2024-03-22)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Version bump for release ([`cdb582d`](https://github.com/berdandy/chatr/commit/cdb582dab938378ab2a58df7ba4d9c188e310ab4))
    - Changed chatr to output css class instead of embedded style ([`35ff5f5`](https://github.com/berdandy/chatr/commit/35ff5f5f8dc6df22ba59a02990846844d0137f71))
</details>

## v0.6.3 (2024-03-21)

<csr-id-7ad98279299e299fd3500793e3ea925cb71e99a8/>

### Chore

 - <csr-id-7ad98279299e299fd3500793e3ea925cb71e99a8/> Release chatr version 0.6.3

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.3 ([`7ad9827`](https://github.com/berdandy/chatr/commit/7ad98279299e299fd3500793e3ea925cb71e99a8))
    - Added pet icons ([`aa01b99`](https://github.com/berdandy/chatr/commit/aa01b99b425b9ba63147031e6f500f095eb31e63))
</details>

## v0.6.2 (2024-03-20)

<csr-id-f71c279bda5458145730ea9c08321656bdfeff7a/>

### Chore

 - <csr-id-f71c279bda5458145730ea9c08321656bdfeff7a/> Release chatr version 0.6.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.2 ([`f71c279`](https://github.com/berdandy/chatr/commit/f71c279bda5458145730ea9c08321656bdfeff7a))
    - Ugh changelog dumbness with automated releases ([`93a0915`](https://github.com/berdandy/chatr/commit/93a0915d191d335465a0adde37dee81e80420616))
    - Actually, I do want to publish to crates ([`b524921`](https://github.com/berdandy/chatr/commit/b5249215780e023ee13cba95d79c781be244c601))
</details>

## v0.6.1 (2024-03-20)

<csr-id-8c9b566f8d3fb216070c776cbae2b4f3f2d4faa4/>
<csr-id-c9df56ebcb59fc31d9a5e07bf1347bb2d74302b3/>

### Chore

 - <csr-id-8c9b566f8d3fb216070c776cbae2b4f3f2d4faa4/> Release chatr version 0.6.1
 - <csr-id-c9df56ebcb59fc31d9a5e07bf1347bb2d74302b3/> added automated release packaging

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 198 calendar days.
 - 223 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release chatr version 0.6.1 ([`8c9b566`](https://github.com/berdandy/chatr/commit/8c9b566f8d3fb216070c776cbae2b4f3f2d4faa4))
    - Duh version # ([`8c15905`](https://github.com/berdandy/chatr/commit/8c1590524676aefb0028e43162c2079df0ffed92))
    - Nopublish ([`0f275ba`](https://github.com/berdandy/chatr/commit/0f275baa8c5f14aa7b67b29b26760b562a3f8b90))
    - Added changelog ([`958c741`](https://github.com/berdandy/chatr/commit/958c7410f9cb54962f76bd24a199f26d2880adea))
    - Added automated release packaging ([`c9df56e`](https://github.com/berdandy/chatr/commit/c9df56ebcb59fc31d9a5e07bf1347bb2d74302b3))
    - Adding info for crates.io ([`fe56769`](https://github.com/berdandy/chatr/commit/fe56769300e54530b32608cfb5735356a820ba87))
    - Version ([`344a8b4`](https://github.com/berdandy/chatr/commit/344a8b4bdd3be981797c1815766e74d8d17cf2da))
    - Added bidirection support ([`bb0cd1c`](https://github.com/berdandy/chatr/commit/bb0cd1c8ec553c1c74ae5c945a10f642f985c163))
    - Added bidirectional chatlink conversion ([`d57d13a`](https://github.com/berdandy/chatr/commit/d57d13ae3c640b08d1bdb2b2fdd6d70afd7758e4))
    - Update README.md ([`60e5e16`](https://github.com/berdandy/chatr/commit/60e5e1665b5a179c44ab22814142d3e80637c753))
    - Bump mio from 0.8.10 to 0.8.11 ([`3d8f5af`](https://github.com/berdandy/chatr/commit/3d8f5afbf2ba2a40ec980b49e11543885dd8514c))
    - Refactoring ([`5663ab7`](https://github.com/berdandy/chatr/commit/5663ab78931ac403a494889075e8266ab811afc8))
    - Librarification ([`48e7933`](https://github.com/berdandy/chatr/commit/48e793386fc291d72da1dbe76d6f9ada50556ae3))
    - Bump h2 from 0.3.22 to 0.3.24 ([`ed29add`](https://github.com/berdandy/chatr/commit/ed29addce500825a3ac3608d1800b758e4d64793))
    - Finalized support for hardcoded alliance legend missing from API. Includes tests ([`b6f1a8e`](https://github.com/berdandy/chatr/commit/b6f1a8ef2a9e839b75bf0ff6d44b5fce3ceb8d9e))
    - Added hardcoded alliance stance support, because of missing api data. ([`4b1c5b5`](https://github.com/berdandy/chatr/commit/4b1c5b51e1086786667065ecde24cca8c496746f))
    - Revert "Used serde_as to transform array-of-tuples to map instead of making map myself" ([`ece766f`](https://github.com/berdandy/chatr/commit/ece766f96f1934e4af23c6e47b6734b1869b2b33))
    - Used serde_as to transform array-of-tuples to map instead of making map myself ([`1c30bd3`](https://github.com/berdandy/chatr/commit/1c30bd3a0753aeec4f3684fa80fe24d91f487c52))
    - Added skill debug output to -d flag ([`224a5b8`](https://github.com/berdandy/chatr/commit/224a5b8b3defe31c8c119c9052a5c31f14fed935))
    - Vim retab all source (to fix tab/space garbage from cross-platform misconfig) ([`b53144a`](https://github.com/berdandy/chatr/commit/b53144a86c99306ba94a27be0776af1989a2e5d0))
    - Missing test case; string cleanup ([`bb28e2f`](https://github.com/berdandy/chatr/commit/bb28e2fccc01abc68f0c9c99e3c41ec962d7a63b))
    - Cleanup, rename, docs for clarity ([`f798960`](https://github.com/berdandy/chatr/commit/f798960a7ff3f015233c003826eff394b2d85577))
    - Changed chatbuildr module to generate strings ([`d14dae7`](https://github.com/berdandy/chatr/commit/d14dae76205362a8fbd52cad22839c17fff3c0d3))
    - Cleanup of old debug prints and version bump that I forgot ([`0138b30`](https://github.com/berdandy/chatr/commit/0138b3017956aeb1c490e3c4ece3a046e0ae6bb8))
    - Added explicit endianness ([`8d671b0`](https://github.com/berdandy/chatr/commit/8d671b037fc8651cf0d5bd3db2c72e0d9bea0e4a))
    - More rust unwrap cleanup ([`571baf8`](https://github.com/berdandy/chatr/commit/571baf843ce8f3740bd1547a7a99dd4adce9eab6))
    - Slightly better rust code ([`a93b711`](https://github.com/berdandy/chatr/commit/a93b711aad1578c2d195869fb653a73d3e110470))
    - Bump openssl from 0.10.57 to 0.10.62 ([`d44efd6`](https://github.com/berdandy/chatr/commit/d44efd6f17bc0332e2eb0d851f4f7128818382f5))
    - Added support for revenant acquirign skills from legend palette rather than from chat code palette ([`ea2e9db`](https://github.com/berdandy/chatr/commit/ea2e9dbf3931d574c2a1f5df2684b67214018758))
    - Updated dependency versions ([`2457ca9`](https://github.com/berdandy/chatr/commit/2457ca9014d327c84e638de988498da5d27489e0))
    - Added soto support (partial) ([`5aa4725`](https://github.com/berdandy/chatr/commit/5aa4725621520263895e46f3b08a2f8b46335692))
    - Update README.md ([`0466476`](https://github.com/berdandy/chatr/commit/046647637e4e19da1593885be0549b3c8ba944cd))
</details>

## v0.2.1 (2023-08-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 38 calendar days.
 - 95 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Better error handling for incomplete pets & legends ([`92a7bc7`](https://github.com/berdandy/chatr/commit/92a7bc7943b37da9dd3e6ec7b5a212b5f59e49c5))
    - Create LICENSE.md ([`b1b5665`](https://github.com/berdandy/chatr/commit/b1b566513e05dd86aabc529249baae8655079761))
    - Changing my mind about license ([`ebbb041`](https://github.com/berdandy/chatr/commit/ebbb0411f205c91108b81b037f775d955860f4e4))
    - Create LICENSE ([`bcf140b`](https://github.com/berdandy/chatr/commit/bcf140b596b174ee3f87b775dc77e86f862e60c6))
    - Update README.md ([`3d2ae06`](https://github.com/berdandy/chatr/commit/3d2ae06dd21ef6b67c537e4581506efade22d2e8))
    - Bump openssl from 0.10.52 to 0.10.55 ([`6e1426c`](https://github.com/berdandy/chatr/commit/6e1426cb109b0e0e7127fd53b783f389f60b9192))
</details>

## v0.2.0 (2023-05-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 367 calendar days.
 - 367 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Updated libs to latest ([`a254493`](https://github.com/berdandy/chatr/commit/a254493170767c99c2b63f1a4f187cf61e04b676))
    - Added support for ranger pets and revenant legends ([`73707de`](https://github.com/berdandy/chatr/commit/73707de917086fd701289924c2f752f35c3e0f39))
    - Bump bumpalo from 3.9.1 to 3.12.0 ([`1bc442e`](https://github.com/berdandy/chatr/commit/1bc442e915bc12e7c433bb74155d0e6dee938bcc))
    - Bump openssl from 0.10.40 to 0.10.48 ([`7eb1528`](https://github.com/berdandy/chatr/commit/7eb15285d7c6826e4a3af74e47be2b7122d69631))
    - Bump tokio from 1.18.1 to 1.26.0 ([`0c223a7`](https://github.com/berdandy/chatr/commit/0c223a7e1115e47db1a0709d541b8c7bac71198a))
    - Added some minimal unit tests ([`1e73252`](https://github.com/berdandy/chatr/commit/1e73252b72f033780b4e15bb9abc91027ff61a6b))
    - Update README.md ([`c4ef070`](https://github.com/berdandy/chatr/commit/c4ef070d2a2efe3235dddff7815a8e6f5639bd89))
    - Update README.md ([`e2774a2`](https://github.com/berdandy/chatr/commit/e2774a2edb02a4b567f6886e0d2dbb0152fb38fc))
</details>

## v0.1.0 (2022-05-04)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 24 calendar days.
 - 24 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Removed script post-tag, and bumped version ([`be54732`](https://github.com/berdandy/chatr/commit/be547323be2d0bc8d50644094672ae014e47f183))
    - Update README.md ([`c09f49e`](https://github.com/berdandy/chatr/commit/c09f49ea7b3e7d5fe233e2e3faa45a0b283c735d))
    - Update README.md ([`e6760a8`](https://github.com/berdandy/chatr/commit/e6760a8db048bd4533beaacac36b8194bf7a8e0b))
</details>

## v0.0.3 (2022-04-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bumped version ([`22676df`](https://github.com/berdandy/chatr/commit/22676dfff538ef2ac974dd811052be33b0feada4))
    - Added zero-effort paste model for website ([`f58ed38`](https://github.com/berdandy/chatr/commit/f58ed3817380d010c7c90d7a2115d74e91e8bd3b))
    - Update README.md ([`695a72c`](https://github.com/berdandy/chatr/commit/695a72c36e4292efae750a8277993ca76b00cd32))
    - Update README.md ([`5fc71b6`](https://github.com/berdandy/chatr/commit/5fc71b6c059910807a8572951c4add3b1ef4ce57))
</details>

## v0.0.2 (2022-04-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Never mind. Revenant skills work just fine. ([`f2734d6`](https://github.com/berdandy/chatr/commit/f2734d623e7d9354de2dfd4fef1f7e1cd67dd709))
    - Added skill support (for everyone except revenants) ([`db992e7`](https://github.com/berdandy/chatr/commit/db992e73dc4ce9084dd150fbdae909c9100d3fa1))
    - Moved cpp reference file to a docs dir ([`9e4ae17`](https://github.com/berdandy/chatr/commit/9e4ae17b69ca04712b102dcf960a4370c7437f24))
    - Added deku fields for ranger and revenant ([`bb854f8`](https://github.com/berdandy/chatr/commit/bb854f85435ee387c9a9ecbd06dd4f1202bdc1f7))
    - Update README.md ([`8ba9921`](https://github.com/berdandy/chatr/commit/8ba99219baf3e6368930261d3b4dabd60df4d4a0))
    - Update README.md ([`826e1d6`](https://github.com/berdandy/chatr/commit/826e1d664445a89317a91aff379154e89cd1830d))
</details>

## v0.0.1 (2022-04-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 1 calendar day.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update README.md ([`08daf38`](https://github.com/berdandy/chatr/commit/08daf38e1f8e59822694cd2ce13c6b694f2a1246))
    - Update README.md ([`1808711`](https://github.com/berdandy/chatr/commit/18087112566e44fe5882dbce1087672074e0e0a7))
    - Added auto-trimming of `[&` and `]` characters ([`2fa90ff`](https://github.com/berdandy/chatr/commit/2fa90ff24231e501a2023dc4866ccac84c3c6b65))
    - Update README.md ([`5d81a58`](https://github.com/berdandy/chatr/commit/5d81a58734b36277862043f9c721170eaec79c5e))
    - Update README.md ([`42da30f`](https://github.com/berdandy/chatr/commit/42da30fca44e6ff41568e66213d660f6a42b47f5))
    - Merge branch 'main' of github.com:accessibilitywars/chatr into main ([`221cc36`](https://github.com/berdandy/chatr/commit/221cc368b335fcbe5cae7ef29787e51ece1828d9))
    - Standardized format, so we can use it for a template system with named parameters ([`ba7dc00`](https://github.com/berdandy/chatr/commit/ba7dc0036c6d4b11e25eeadc98a78769b0a66627))
    - Update README.md ([`f70b65f`](https://github.com/berdandy/chatr/commit/f70b65f55d4c1e2a5b3e8ad26a84ff30872c606f))
    - Update README.md ([`06c8fae`](https://github.com/berdandy/chatr/commit/06c8fae52bbfe4e18336a47e6e3baf0f7d5eeb38))
    - Updated readme and todo ([`24024b9`](https://github.com/berdandy/chatr/commit/24024b9155dfb2c30ef69e2049145dc198ef25d5))
    - Working implementation of chat translation to armory-embed format ([`031107c`](https://github.com/berdandy/chatr/commit/031107c4a43402c5434a41508799e7100cc9e96e))
    - TODO updated ([`83e2fe2`](https://github.com/berdandy/chatr/commit/83e2fe2aa7f27971ce801eb3f9cf43f7f78e9d2d))
    - Added trait index lookup ([`423350e`](https://github.com/berdandy/chatr/commit/423350efb0743f6d25efe5906501ac904206f407))
    - As suggested, versioning Cargo.lock for executable (ignored for libraries) ([`ef8091c`](https://github.com/berdandy/chatr/commit/ef8091c9b61c81e6079106af10d8f609fb8c977e))
    - Added vscode stuff ([`5be8379`](https://github.com/berdandy/chatr/commit/5be8379ac402047eec264afcbc2fbc17ea2a1dbf))
    - Update README.md ([`21c547f`](https://github.com/berdandy/chatr/commit/21c547fc058cbe471be19fd63cae63250e2b9be0))
    - Added base64 decode sample code ([`1ebc612`](https://github.com/berdandy/chatr/commit/1ebc612e0f3e03ed9c67d9f6c37f840309249344))
    - Initial version (just super rough commandline handling) ([`c3c38c8`](https://github.com/berdandy/chatr/commit/c3c38c8d10672856dd70230fb2fb832615d19b00))
    - Initial commit ([`8365123`](https://github.com/berdandy/chatr/commit/8365123d1dbd409907e7dbc9c470e8a7e120cf15))
</details>

