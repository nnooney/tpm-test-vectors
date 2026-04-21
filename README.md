# tpm-test-vectors

## About

Test vectors for validating TPM implementations.

The test vectors themselves are expressed using Rusty Object Notation
([`ron`](https://github.com/ron-rs/ron)) and located in
[`test-vectors/data`](test-vectors/data/).

The `test-vectors` crate also contains code to issue the test vectors against a
TPM implementation, using the `tpm2-client` crate from the
[`tpm-rs`](https://github.com/tpm-rs/tpm-rs) project.

## Test Vector Details

### Pass / Fail

Rust's `cargo test` framework does not support skipping tests at runtime; it only
reports pass or fail. Consequently, test vectors for features unsupported by the
target TPM will report as **failed**.

Do not expect all tests to pass unless the TPM implementation supports every
command, feature, and algorithm in the specification.

### TPM Specification

These test vectors are written against v184 of the TPM specification.

https://trustedcomputinggroup.org/resource/tpm-library-specification/

### Initial TPM state

Each test vector is designed to be run against a TPM that has just been
initialized (`_TPM_Init`), but has not yet been started. No expectations are
made on the non-volatile storage in the TPM.

### Testing Failed Commands

The TPM specification states the following (formatted to fit in block quotes):

> When a command fails to complete for any reason, the TPM shall return
>
> - a TPM_ST (UINT16) with a value of TPM_TAG_RSP_COMMAND or TPM_ST_NO_SESSIONS
>   followed by
> - a UINT32 (responseSize) with a value of 10, followed by
> - a UINT32 containing a response code with a value other than TPM_RC_SUCCESS.

> If the tag of the command is not a recognized command tag, the TPM error
> response will differ depending on TPM 1.2 compatibility. If the TPM supports
> 1.2 compatibility, the TPM shall return a tag of TPM_TAG_RSP_COMMAND and an
> appropriate TPM 1.2 response code (TPM_BADTAG = 00 00 00 1E16). If the TPM
> does not have compatibility with TPM 1.2, the TPM shall return
> TPM_ST_NO_SESSION and a response code of TPM_RC_TAG.

The test vectors assume that there is no TPM 1.2 compatibility, meaning they
expect `TPM_ST_NO_SESSIONS` as the tag returned by failed commands.

## Status

The large checklist below describes the sections from v184 of the TPM
specification (Part 3: Commands). This can be used to get a sense of coverage of
the test vectors.

<details>

<summary>Click to show checklist</summary>

- [ ] 5 Command Processing
  - [ ] 5.1 Introduction
  - [x] 5.2 Command Header Validation
  - [x] 5.3 Mode Checks
  - [ ] 5.4 Handle Area Validation
  - [ ] 5.5 Session Area Validation
  - [ ] 5.6 Authorization Checks
  - [ ] 5.7 Parameter Decryption
  - [ ] 5.8 Parameter Unmarshaling
    - [ ] 5.8.1 Introduction
    - [ ] 5.8.2 Unmarshaling Errors
  - [ ] 5.9 Command Post Processing
- [ ] 6 Response Values
  - [ ] 6.1 Tag
  - [ ] 6.2 Response Codes
- [ ] 7 Implementation Dependent
- [ ] 8 Detailed Actions Assumptions
  - [ ] 8.1 Introduction
  - [ ] 8.2 Pre-processing
  - [ ] 8.3 Post Processing
- [ ] 9 Start-up
  - [ ] 9.1 Introduction
  - [ ] 9.2 \_TPM_Init
  - [ ] 9.3 TPM2_Startup
  - [ ] 9.4 TPM2_Shutdown
- [ ] 10 Testing
  - [ ] 10.1 Introduction
  - [ ] 10.2 TPM2_SelfTest
  - [ ] 10.3 TPM2_IncrementalSelfTest
  - [ ] 10.4 TPM2_GetTestResult
- [ ] 11 Session Commands
  - [ ] 11.1 TPM2_StartAuthSession
  - [ ] 11.2 TPM2_PolicyRestart
- [ ] 12 Object Commands
  - [ ] 12.1 TPM2_Create
  - [ ] 12.2 TPM2_Load
  - [ ] 12.3 TPM2_LoadExternal
  - [ ] 12.4 TPM2_ReadPublic
  - [ ] 12.5 TPM2_ActivateCredential
  - [ ] 12.6 TPM2_MakeCredential
  - [ ] 12.7 TPM2_Unseal
  - [ ] 12.8 TPM2_ObjectChangeAuth
  - [ ] 12.9 TPM2_CreateLoaded
- [ ] 13 Duplication Commands
  - [ ] 13.1 TPM2_Duplicate
  - [ ] 13.2 TPM2_Rewrap
  - [ ] 13.3 TPM2_Import
- [ ] 14 Asymmetric Primitives
  - [ ] 14.1 Introduction
  - [ ] 14.2 TPM2_RSA_Encrypt
  - [ ] 14.3 TPM2_RSA_Decrypt
  - [ ] 14.4 TPM2_ECDH_KeyGen
  - [ ] 14.5 TPM2_ECDH_ZGen
  - [ ] 14.6 TPM2_ECC_Parameters
  - [ ] 14.7 TPM2_ZGen_2Phase
  - [ ] 14.8 TPM2_ECC_Encrypt
  - [ ] 14.9 TPM2_ECC_Decrypt
- [ ] 15 Symmetric Primitives
  - [ ] 15.1 Introduction
  - [ ] 15.2 TPM2_EncryptDecrypt
  - [ ] 15.3 TPM2_EncryptDecrypt2
  - [ ] 15.4 TPM2_Hash
  - [ ] 15.5 TPM2_HMAC
  - [ ] 15.6 TPM2_MAC
- [ ] 16 Random Number Generator
  - [ ] 16.1 TPM2_GetRandom
  - [ ] 16.2 TPM2_StirRandom
- [ ] 17 Hash/HMAC/Event Sequences
  - [ ] 17.1 Introduction
  - [ ] 17.2 TPM2_HMAC_Start
  - [ ] 17.3 TPM2_MAC_Start
  - [ ] 17.4 TPM2_HashSequenceStart
  - [ ] 17.5 TPM2_SequenceUpdate
  - [ ] 17.6 TPM2_SequenceComplete
  - [ ] 17.7 TPM2_EventSequenceComplete
- [ ] 18 Attestation Commands
  - [ ] 18.1 Introduction
  - [ ] 18.2 TPM2_Certify
  - [ ] 18.3 TPM2_CertifyCreation
  - [ ] 18.4 TPM2_Quote
  - [ ] 18.5 TPM2_GetSessionAuditDigest
  - [ ] 18.6 TPM2_GetCommandAuditDigest
  - [ ] 18.7 TPM2_GetTime
  - [ ] 18.8 TPM2_CertifyX509
- [ ] 19 Ephemeral EC Keys
  - [ ] 19.1 Introduction
  - [ ] 19.2 TPM2_Commit
  - [ ] 19.3 TPM2_EC_Ephemeral
- [ ] 20 Signing and Signature Verification
  - [ ] 20.1 TPM2_VerifySignature
  - [ ] 20.2 TPM2_Sign
- [ ] 21 Command Audit
  - [ ] 21.1 Introduction
  - [ ] 21.2 TPM2_SetCommandCodeAuditStatus
- [ ] 22 Integrity Collection (PCR)
  - [ ] 22.1 Introduction
  - [ ] 22.2 TPM2_PCR_Extend
  - [ ] 22.3 TPM2_PCR_Event
  - [ ] 22.4 TPM2_PCR_Read
  - [ ] 22.5 TPM2_PCR_Allocate
  - [ ] 22.6 TPM2_PCR_SetAuthPolicy
  - [ ] 22.7 TPM2_PCR_SetAuthValue
  - [ ] 22.8 TPM2_PCR_Reset
  - [ ] 22.9 \_TPM_Hash_Start
    - [ ] 22.9.1 Description
  - [ ] 22.10 \_TPM_Hash_Data
    - [ ] 22.10.1 Description
  - [ ] 22.11 \_TPM_Hash_End
    - [ ] 22.11.1 Description
- [ ] 23 Enhanced Authorization (EA) Commands
  - [ ] 23.1 Introduction
  - [ ] 23.2 Signed Authorization Actions
    - [ ] 23.2.1 Introduction
    - [ ] 23.2.2 Policy Parameter Checks
    - [ ] 23.2.3 Policy Digest Update Function (PolicyUpdate())
    - [ ] 23.2.4 Policy Context Updates
    - [ ] 23.2.5 Policy Ticket Creation
  - [ ] 23.3 TPM2_PolicySigned
  - [ ] 23.4 TPM2_PolicySecret
  - [ ] 23.5 TPM2_PolicyTicket
  - [ ] 23.6 TPM2_PolicyOR
  - [ ] 23.7 TPM2_PolicyPCR
  - [ ] 23.8 TPM2_PolicyLocality
  - [ ] 23.9 TPM2_PolicyNV
  - [ ] 23.10 TPM2_PolicyCounterTimer
  - [ ] 23.11 TPM2_PolicyCommandCode
  - [ ] 23.12 TPM2_PolicyPhysicalPresence
  - [ ] 23.13 TPM2_PolicyCpHash
  - [ ] 23.14 TPM2_PolicyNameHash
  - [ ] 23.15 TPM2_PolicyDuplicationSelect
  - [ ] 23.16 TPM2_PolicyAuthorize
  - [ ] 23.17 TPM2_PolicyAuthValue
  - [ ] 23.18 TPM2_PolicyPassword
  - [ ] 23.19 TPM2_PolicyGetDigest
  - [ ] 23.20 TPM2_PolicyNvWritten
  - [ ] 23.21 TPM2_PolicyTemplate
  - [ ] 23.22 TPM2_PolicyAuthorizeNV
  - [ ] 23.23 TPM2_PolicyCapability
  - [ ] 23.24 TPM2_PolicyParameters
  - [ ] 23.25 TPM2_PolicyTransportSPDM
- [ ] 24 Hierarchy Commands
  - [ ] 24.1 TPM2_CreatePrimary
  - [ ] 24.2 TPM2_HierarchyControl
  - [ ] 24.3 TPM2_SetPrimaryPolicy
  - [ ] 24.4 TPM2_ChangePPS
  - [ ] 24.5 TPM2_ChangeEPS
  - [ ] 24.6 TPM2_Clear
  - [ ] 24.7 TPM2_ClearControl
  - [ ] 24.8 TPM2_HierarchyChangeAuth
  - [ ] 24.9 TPM2_ReadOnlyControl
- [ ] 25 Dictionary Attack Functions
  - [ ] 25.1 Introduction
  - [ ] 25.2 TPM2_DictionaryAttackLockReset
  - [ ] 25.3 TPM2_DictionaryAttackParameters
- [ ] 26 Miscellaneous Management Functions
  - [ ] 26.1 Introduction
  - [ ] 26.2 TPM2_PP_Commands
  - [ ] 26.3 TPM2_SetAlgorithmSet
- [ ] 27 Field Upgrade
  - [ ] 27.1 Introduction
  - [ ] 27.2 TPM2_FieldUpgradeStart
  - [ ] 27.3 TPM2_FieldUpgradeData
  - [ ] 27.4 TPM2_FirmwareRead
- [ ] 28 Context Management
  - [ ] 28.1 Introduction
  - [ ] 28.2 TPM2_ContextSave
  - [ ] 28.3 TPM2_ContextLoad
  - [ ] 28.4 TPM2_FlushContext
  - [ ] 28.5 TPM2_EvictControl
- [ ] 29 Clocks and Timers
  - [ ] 29.1 TPM2_ReadClock
  - [ ] 29.2 TPM2_ClockSet
  - [ ] 29.3 TPM2_ClockRateAdjust
- [ ] 30 Capability Commands
  - [ ] 30.1 Introduction
  - [ ] 30.2 TPM2_GetCapability
  - [ ] 30.3 TPM2_TestParms
  - [ ] 30.4 TPM2_SetCapability
- [ ] 31 Non-volatile Storage
  - [ ] 31.1 Introduction
  - [ ] 31.2 NV Counters
  - [ ] 31.3 TPM2_NV_DefineSpace
  - [ ] 31.4 TPM2_NV_UndefineSpace
  - [ ] 31.5 TPM2_NV_UndefineSpaceSpecial
  - [ ] 31.6 TPM2_NV_ReadPublic
  - [ ] 31.7 TPM2_NV_Write
  - [ ] 31.8 TPM2_NV_Increment
  - [ ] 31.9 TPM2_NV_Extend
  - [ ] 31.10 TPM2_NV_SetBits
  - [ ] 31.11 TPM2_NV_WriteLock
  - [ ] 31.12 TPM2_NV_GlobalWriteLock
  - [ ] 31.13 TPM2_NV_Read
  - [ ] 31.14 TPM2_NV_ReadLock
  - [ ] 31.15 TPM2_NV_ChangeAuth
  - [ ] 31.16 TPM2_NV_Certify
  - [ ] 31.17 TPM2_NV_DefineSpace2
  - [ ] 31.18 TPM2_NV_ReadPublic2
- [ ] 32 Attached Components
  - [ ] 32.1 Introduction
  - [ ] 32.2 TPM2_AC_GetCapability
  - [ ] 32.3 TPM2_AC_Send
  - [ ] 32.4 TPM2_Policy_AC_SendSelect
- [ ] 33 Authenticated Countdown Timer
  - [ ] 33.1 Introduction
  - [ ] 33.2 TPM2_ACT_SetTimeout
- [ ] 34 Vendor Specific
  - [ ] 34.1 Introduction
  - [ ] 34.2 TPM2_Vendor_TCG_Test

</details>

### Test Notes

- The following vectors test all commands meeting certain criteria and must be
  updated as new commands are introduced in later versions of the specification:
  - [`0004-failure-mode-command-not-allowed.ron`](test-vectors/src/vectors/0004-failure-mode-command-not-allowed.ron)
  - [`0009-session-area-bad-sessions-tag.ron`](test-vectors/src/vectors/0009-session-area-bad-sessions-tag.ron)
  - [`0010-session-area-bad-no-sessions-tag.ron`](test-vectors/src/vectors/0010-session-area-bad-no-sessions-tag.ron)
- 5.4 Handle Area Validation applies additional edge cases to all commands that
  use handles.
  - Wrong number of handles passed to command (0, -1, +1)
  - Value of handle is not consistent with command syntax
  - Handle for object not loaded in the TPM
  - Test for handle in each hierarchy

## Building & Testing

```
# Just build the project
cargo build
```

There are several types of tests provided:

- Unit tests: ensures the library code functions
- Integration tests: runs the test vector against a TPM implementation.

```
# Run unit tests only
cargo test --lib

# Run all tests
cargo test
```

### Integration tests

Each file in the [`test-vectors/tests`](test-vectors/tests) folder represents a
TPM target used to run the test vectors against. They use environment variables
to configure sending the test vectors to the TPM.

#### Simulator

Test vectors run against the TCG reference TPM simulator in
([`test-vectors/tests/simulator.rs`](test-vectors/tests/simulator.rs)).

The following environment variables and defaults are used:

| Environment Variable | Default           | Description                       |
| -------------------- | ----------------- | --------------------------------- |
| `SIMULATOR_BIN`      | `/tpm2-simulator` | Program to run the TPM simulator. |
| `SIMULATOR_IP`       | `127.0.0.1`       | IP address to connect to.         |
| `SIMULATOR_ARGS`     |                   | Arguments for the TPM simulator.  |

This repo also provides a Docker compose file which builds the TCG reference TPM
simulator and runs the test vectors against it.

```shell
# Run one of these commands from the test-vectors crate root
docker compose run --rm simulator_tests
podman compose run --rm simulator_tests
```
