# DiskScope Architecture

## Purpose

DiskScope is a high-performance filesystem analysis engine focused on helping users understand disk usage through fast scanning, clear visualization, and actionable insights.

The project is designed around a reusable engine that can power multiple clients including desktop and mobile applications.

---

# Architecture Overview

DiskScope consists of two primary parts.

- Engine
- Desktop

Future versions may introduce additional clients without modifying the engine.

Example

Desktop
    │
    ▼
Engine

Future

Desktop
Mobile
CLI
    │
    ▼
Engine

---

# Responsibilities

## Engine

Responsible for:

- Filesystem traversal
- Metadata collection
- Tree construction
- Size aggregation
- Analysis
- Search
- Platform abstraction

The engine must never depend on any user interface.

---

## Desktop

Responsible for:

- Rendering
- User interaction
- Progress visualization
- Settings

Desktop must never contain business logic.

---

# Dependency Direction

Desktop
    │
    ▼
Engine

Never:

Engine
    │
    ▼
Desktop

---

# Design Principles

- Engine First
- Platform Independent
- Business Logic Isolation
- Information First
- Performance Before Decoration

---

# Future Roadmap

v0

Recursive Scanner

v1

Desktop

v2

Performance

v3

Cross Platform

v4

Mobile