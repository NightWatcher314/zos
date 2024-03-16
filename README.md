# ZOS

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Contents](#contents)

## About <a name = "about"></a>

本项目为个人的 rcore 学习经历，目前已经支持在裸机上运行一个支持虚拟地址空间的时间片轮转分时操作系统。

## Getting Started <a name = "getting_started"></a>

### Prerequisites

你需要安装rust的配套工具链来运行代码

### Run

```bash
git clone https://github.com/NightWatcher314/zos.git
cd os
make run
```
即可自动运行 user 目录下的几个测试用例





## Contents <a name = "contents"></a>
os 目录的大致模块如下
```bash
❯ tree
.
├── boards
│   └── qemu.rs
├── config.rs
├── console.rs
├── entry.asm
├── lang_items.rs
├── link_app.S
├── linker-qemu.ld
├── loader.rs # 用于载入 user 下的代码段
├── main.rs 
├── mm # 用于虚拟地址空间
│   ├── address.rs
│   ├── frame_allocator.rs
│   ├── heap_allocator.rs
│   ├── memory_set.rs
│   ├── mod.rs
│   └── page_table.rs
├── sbi.rs
├── syscall # 用于支持系统调用
│   ├── fs.rs
│   ├── mod.rs
│   └── process.rs
├── task # 用于支持任务切换以及调度
│   ├── context.rs
│   ├── mod.rs
│   ├── switch.S
│   ├── switch.rs
│   └── task.rs
├── timer.rs # 计时器
├── trap # 用于支持trap功能
│   ├── context.rs
│   ├── mod.rs
│   └── trap.S
└── utils
    ├── mod.rs
    └── up.rs
```


目前 user 目录中的几个测试用例为
```bash
❯ tree
.
├── 00power_3.rs
├── 01power_5.rs
├── 02power_7.rs
├── 03sleep.rs
├── 04load_fault.rs
├── 05store_fault.rs
└── sbrk_test.rs

```
分别测试了基本的运算、时钟中断相关的系统调用、虚拟地址空间的访存以及相关的保护
