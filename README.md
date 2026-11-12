# `tyyoj-contracts`

Shared Rust library and OpenAPI specifications for the **Toi Yêu Yuri Online Judge (`tyyoj`)** platform.

## Overview

This repository defines the single source of truth for all data transfer objects (DTOs), API schemas, submission verdicts, language identifiers, and judge mode configurations used across `tyyoj-backend`, `tyyoj-judger`, and client code generators.

## Core Definitions

- **Submission Verdicts:**
  `PENDING`, `COMPILING`, `RUNNING`, `ACCEPTED`, `WRONG_ANSWER`, `TIME_LIMIT_EXCEEDED`, `MEMORY_LIMIT_EXCEEDED`, `RUNTIME_ERROR`, `COMPILATION_ERROR`, `IDLENESS_LIMIT_EXCEEDED`.
- **Submission Languages:**
  `Cpp20` (`g++`), `Pascal` (`fpc`), `Python3` (`python3`/`pypy3`).
- **Judge Modes:**
  `Standard`, `CustomChecker`, `Interactive`.
- **OpenAPI Specifications:**
  Located in `specs/openapi.yaml`.

## License

MIT © 2026 Toi Yêu Yuri Online Judge (toiyeuyuriiiii & nmkdeveloper)
