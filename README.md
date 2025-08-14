# arm_vcpu

[![CI](https://github.com/arceos-hypervisor/arm_vcpu/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/arceos-hypervisor/arm_vcpu/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/arm_vcpu)](https://crates.io/crates/arm_vcpu)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)]

AArch64 virtual CPU (vCPU) implementation for hypervisors. This crate provides the core vCPU structure and virtualization-related interface support specifically designed for the AArch64 architecture.

## Features

- 🔧 **Complete vCPU Implementation**: Full virtual CPU structure for AArch64 guests
- 🚀 **Exception Handling**: Comprehensive trap and exception handling for virtualized environments
- 🎯 **Hardware Virtualization**: Support for ARMv8 virtualization extensions (EL2)
- 🔒 **Security**: SMC (Secure Monitor Call) handling and secure virtualization
- 📊 **Per-CPU Support**: Efficient per-CPU data structures and management
- 🛠️ **No-std Compatible**: Works in bare-metal and embedded environments

## Architecture Overview

This crate implements the following key components:

- **`Aarch64VCpu`**: The main virtual CPU structure that manages guest execution state
- **`TrapFrame`**: Context frame for handling traps and exceptions from guest VMs  
- **Exception Handlers**: Support for synchronous and asynchronous exception handling
- **System Register Emulation**: Virtualized access to AArch64 system registers
- **SMC Interface**: Secure Monitor Call handling for trusted execution

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
arm_vcpu = "0.1"
```

### Basic Example

```rust
use arm_vcpu::{Aarch64VCpu, Aarch64VCpuCreateConfig, has_hardware_support};

// Check if hardware virtualization is supported
if has_hardware_support() {
    // Create vCPU configuration
    let config = Aarch64VCpuCreateConfig::default();
    
    // Create and configure the virtual CPU
    let vcpu = Aarch64VCpu::new(config)?;
    
    // Run the virtual CPU
    vcpu.run()?;
}
```

## Requirements

- **Architecture**: AArch64 (ARMv8-A or later)
- **Privilege Level**: EL2 (Hypervisor mode) required for full functionality

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

#### 2. **关键模块解析**

|     模块类别     |           代表节点           |            技术功能说明             |
| :--------------: | :--------------------------: | :---------------------------------: |
| **核心硬件抽象** |      `x86_64`, `riscv`       |       不同CPU架构的底层适配层       |
|   **内存管理**   |     `axaddrspace 0.1.0`      | 虚拟地址空间管理模块(0.1.0表示版本) |
|  **寄存器操作**  | `lock_registers`, `volatile` |   硬件寄存器原子操作与内存映射I/O   |
|  **序列化系统**  |   `serde`, `winnow 0.6.26`   |     数据序列化框架(含特定版本)      |
|   **并发控制**   |         `tiny_lock`          |            轻量级锁实现             |
|  **初始化系统**  |          `lazyinit`          |          延迟初始化管理器           |

#### 3. **依赖关系技术细节**