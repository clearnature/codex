# 构建与版本号（操作手册）

> 状态：**已落地，端到端验证通过**（2026-09-15 ~ 09-16）。
> 基线：`feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。
> 关联：[`../maps/tech-stack.md`](../maps/tech-stack.md)、[`../maps/references.md`](../maps/references.md)、[`i18n-verification.md`](./i18n-verification.md)。

## 〇、快速开始（下次照这个做）

### 编 glibc release（会携带真实版本号）

```bash
scripts/build-release-local.sh                 # 版本自动取自最近的 rust-v* tag
scripts/build-release-local.sh --version 0.154.0
```

### 编 musl release（官方 Linux 发行目标）

```bash
scripts/setup-musl-toolchain.sh                 # 缺 apt 包时会打印唯一那条 sudo 命令
set -a; . /tmp/codex-musl-env-x86_64-unknown-linux-musl.sh; set +a
scripts/build-release-local.sh x86_64-unknown-linux-musl
```

### 只想让普通 cargo 构建别挂在 V8 上

```bash
eval "$(scripts/setup-rusty-v8.sh)"
cargo build -p codex-cli --bin codex
```

### 产出位置

```
codex-rs/target/x86_64-unknown-linux-musl/release/{codex,codex-code-mode-host,codex-responses-api-proxy,bwrap}
codex-rs/target/x86_64-unknown-linux-gnu/release/{...同样四个...}
```

## 一、要解决的三个问题

| #      | 问题                                              | 本质                                                                                                 | 上游态度                                                      |
| ------ | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| **P1** | 编译产物无法携带版本号（恒报 `codex-cli 0.0.0`）  | 版本号**只在发版 tag 上写**，`main` 恒为占位值，且 Cargo 不提供构建期覆盖 `CARGO_PKG_VERSION` 的手段 | issue [#14065]，**已 `not_planned` 关闭**，无 PR              |
| **P2** | 本地 `cargo build` 挂在 `v8 v150.4.0`（HTTP 404） | 上游自建 rusty_v8 产物只喂给 CI，本地无对等入口                                                      | issue [#36698] / [#43318]，**均 open**，社区靠手工 workaround |
| **P3** | musl 目标本地无法构建                             | CI 用一整套 musl 交叉工具链（Zig + 自编译 libcap），本地无文档                                       | 无对应 issue                                                  |

[#14065]: https://github.com/openai/codex/issues/14065
[#36698]: https://github.com/openai/codex/issues/36698
[#43318]: https://github.com/openai/codex/issues/43318

**不要等上游**：P1 被明确标记为 not planned；P2 自 2026-08 至今未修。

## 二、拓扑扫描（实测事实）

### 2.1 版本号的影响面远大于"显示字符串"

`codex-rs/Cargo.toml` 的 `[workspace.package] version` 被 **150 个 crate** 以 `version.workspace = true` 继承，并在 **101 处**以 `env!("CARGO_PKG_VERSION")` 编译期内联。其中三处是**对外契约**：

| 落点                       | 位置                                    | 后果                                 |
| -------------------------- | --------------------------------------- | ------------------------------------ |
| 轮次元数据 `codex_version` | `core/src/turn_metadata.rs:48,255`      | 每次 turn 上报给模型的**客户端身份** |
| HTTP `User-Agent` 前缀     | `login/src/auth/default_client.rs:165`  | 请求指纹                             |
| 遥测 `codex_rs_version`    | `analytics/src/events.rs:1445`、`otel/` | 数据侧区分客户端版本                 |

其余为显示、`codex doctor`、rollout 记录、app-server 协议字段等。

> **推论**：改版本号不是纯装饰性操作——它改变向服务端申报的版本。代价可接受（上游 `main` 自己也报 `0.0.0`），但要**知情**。这也是为什么方案必须动 manifest，而不能只改显示层。

### 2.2 版本号没有任何自动化

```text
写入方：无。全仓没有对 Cargo.toml 的 sed/write 逻辑，justfile 也没有 bump 配方。
读取方：scripts/codex_package/version.py（只读）、rust-release.yml 的 tag 校验门。
```

真实版本只在发版提交里出现一次。tag `rust-v0.154.0` 指向的提交 `6b9826e3a`，**整个提交只改这一行**（`1 file changed, 1 insertion(+), 1 deletion(-)`），且**不在 `main` 上**：

| 位置                         | version   |
| ---------------------------- | --------- |
| `origin/main`                | `0.0.0`   |
| tag `rust-v0.154.0`          | `0.154.0` |
| tag `rust-v0.143.0-alpha.10` | `0.0.0`   |

### 2.3 快照与版本号耦合，且"替换式归一化"救不了它

`tui/src/status/tests.rs:186` 的 `sanitize_directory()` 只归一化目录，**不归一化版本**，渲染出的 `v{版本}` 原样落进快照：24 个 `.snap` 文件、28 处渲染。

看起来 `tui/src/chatwidget/rendering_tests.rs` 给了正确姿势（`.replace(CODEX_CLI_VERSION, "<VERSION>")`），**但它是坏的**——仓库遗留的 `.rendering_tests.rs.pending-snap` 就是实锤：

```text
old: │ >_ OpenAI Codex (v<VERSION>)              │   <- 版本号较长时生成
new: │ >_ OpenAI Codex (v<VERSION>)            │     <- 版本号较短时生成
```

方框按**真实版本号的字符数**算内边距再渲染；替换成不同长度的占位符后尾部 `│` 会移位，快照依然依赖版本号**长度**。想真正解耦必须把版本号参数化进渲染（改生产代码 `status/card.rs`），成本高于收益。

**这条结论直接决定了方案选择**：既然"版本号自由变动"做不到零成本，就不要让工作区的版本号变动 —— 改为**只在构建期注入**。

## 三、多路径推演

### P1 让产物携带版本号

| 路径                            | 做法                                     | 复杂度                        | 同步上游冲突         | 风险                                                     |
| ------------------------------- | ---------------------------------------- | ----------------------------- | -------------------- | -------------------------------------------------------- |
| **① 构建期注入 + 还原**（采用） | 脚本临时写 manifest，构建后 `trap` 还原  | 低                            | **零**（不改源码）   | 构建期工作区被临时改动；并发会互相污染（已用 lock 消除） |
| ② `build.rs` 派生 + 改调用点    | 新增 crate，替换 101 处 `env!`           | 高（46 文件 + `BUILD.bazel`） | 高（每次同步都冲突） | 触及 `codex-core`，被 `AGENTS.md` 明确劝退               |
| ③ 提交时 bump 并一起提交        | 改 manifest + lock + 重录 142 个快照文件 | 中                            | 中                   | 只携带"上次 bump 的版本"；快照噪音极大                   |

### P2 V8

| 路径                   | 做法                                  | 代价                                      |
| ---------------------- | ------------------------------------- | ----------------------------------------- |
| **① 固化脚本**（采用） | `scripts/setup-rusty-v8.sh`，双重校验 | 一次                                      |
| ② 手工 workaround      | 每次照 issue 评论敲一遍               | 每台机器重踩                              |
| ③ `V8_FROM_SOURCE=1`   | 源码构建 V8                           | 数小时，且 #43318 证明会因缺 ICU 数据失败 |

### P3 musl

| 路径                       | 做法                                     | 代价                                      |
| -------------------------- | ---------------------------------------- | ----------------------------------------- |
| **① 复用 CI 逻辑**（采用） | 过滤掉 CI 脚本里的 sudo 行，其余原样执行 | 需一次性 apt 安装（唯一需要 sudo 的地方） |
| ② 自己重写等价逻辑         | 手抄 libcap 编译 + 20 个环境变量         | 与 CI 漂移，难维护                        |

## 四、落地方案

### 4.1 `scripts/setup-rusty-v8.sh` — V8 产物引导

等价于 CI 的 `.github/actions/setup-rusty-v8`：解析 `v8` 版本 → 从 `openai/codex` 的 `rusty-v8-v<版本>` release 取三个文件 → **双重校验**（清单文件的 sha256 对仓库内可信清单；产物对下载的 `.sha256`）→ 输出可直接 `eval` 的 export。

### 4.2 `scripts/build-release-local.sh` — 版本注入 + 官方发布流程

```bash
scripts/build-release-local.sh                       # 宿主 target
scripts/build-release-local.sh --version 0.154.0     # 显式指定
CODEX_BUILD_VERSION=0.155.0 scripts/build-release-local.sh
```

执行序列：

```text
0/4  解析版本：--version / CODEX_BUILD_VERSION / git describe --match 'rust-v[0-9]*'
     取锁（mkdir，可移植；并发第二次直接拒绝退出）
     把 Cargo.toml + Cargo.lock 原始字节备份到 mktemp -d
     安装 trap：EXIT 无条件还原，INT -> 130，TERM -> 143
1/4  按 [workspace.package] 区段定位写入版本，cargo update --workspace 同步 lockfile
2/4  rusty_v8 引导（setup-rusty-v8.sh）
3/4  cargo build --release --bin bwrap -> strip -> CODEX_BWRAP_SHA256
4/4  cargo build --release --bin codex --bin codex-code-mode-host --bin codex-responses-api-proxy
     校验产物自报版本；不符即非 0 退出；最后还原 manifest
```

关键设计（每条都有实测证据，见第五节）：

- **区段定位写入**，不是全局 `sed`，不会误伤依赖段。
- **`trap` 覆盖三条退出路径**：正常结束、构建失败、Ctrl-C / SIGTERM，中断时退出码仍是 130 / 143。
- **mkdir 锁消除并发污染**（`flock` 在 macOS 上不存在，故用 mkdir）。
- **还原是字节级**：md5 比对确认构建前后 `Cargo.toml` / `Cargo.lock` 完全一致。
- **产物自检**：构建后直接执行 `codex --version`，与目标版本不符即非 0 退出。
- **musl 前置检查**：缺 `zig` / `musl-gcc` 时提前报错并指向 `setup-musl-toolchain.sh`。

### 4.3 `scripts/setup-musl-toolchain.sh` — musl 交叉工具链

唯一的 sudo 步骤（脚本会打印，不会自己跑）：

```bash
sudo apt-get update && sudo apt-get install -y \
  ca-certificates curl musl-tools pkg-config libcap-dev \
  g++ clang libc++-dev libc++abi-dev lld xz-utils binutils
```

其余五步脚本自动完成：

```text
1/5  rustup target add <target>   —— 必须 cd 进 codex-rs/ 再跑（见 6.4）
2/5  前置检查 curl/tar/xz/make
3/5  Zig 0.14.0 装到 ~/.local/opt，并软链到 ~/.local/bin/zig
4/5  跑 CI 脚本（过滤掉它的 sudo 行）：把 libcap 2.75 编成 musl 版 + 注入约 20 个环境变量
5/5  把 GITHUB_ENV 转成可直接 source 的 env 文件
```

为什么不能只装 `musl-tools` 了事：

- **Zig 不是 Rust 的 linker**，而是给 native 依赖（`aws-lc-sys` 等）当 C/C++ 编译器；CI 会生成 `zigcc` / `zigcxx` 包装脚本绕开四个坑（丢 Rust 风格 `--target`、`-I/usr/include` 降级成 `-idirafter`、补 `-fno-sanitize=undefined`）。
- **apt 的 `libcap-dev` 是 glibc 链接的，musl 用不了** → 必须用 `musl-gcc` 重编 `libcap.a`（脚本下载 `libcap-2.75.tar.xz` 并校验 sha256）。
- **Rust 的 linker 必须是 `musl-gcc`**（`CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER`），注释写明是为了 _avoid Zig injecting its own CRT_。
- 还要 `AWS_LC_SYS_NO_JITTER_ENTROPY=1`。

### 4.4 CI（待落地）

上游 `rust-release.yml` 的第一道门是 tag == Cargo.toml version。fork 若要走自己的发版，tag 推送时直接调 `scripts/build-release-local.sh <target>`（版本由脚本从 tag 派生）→ 上传产物即可，不需要"记得改版本号"的人肉步骤。日常 CI 也无需额外一致性门：工作区 manifest 恒为 `0.0.0`，构建期注入不会把它改脏。

## 五、验证证据（最终状态）

全部为本机实测输出。

### 5.1 三套产物

| 目标      | 文件                        | 大小   | 自报版本            | 链接方式                           |
| --------- | --------------------------- | ------ | ------------------- | ---------------------------------- |
| **musl**  | `codex`                     | 1.18 G | `codex-cli 0.154.0` | **static-pie / statically linked** |
|           | `codex-code-mode-host`      | 173 M  |                     | static-pie                         |
|           | `codex-responses-api-proxy` | 47 M   |                     | static-pie                         |
|           | `bwrap`                     | 517 K  |                     | static-pie（已 strip）             |
| **glibc** | 同样四个                    | 1.19 G | `codex-cli 0.154.0` | 动态                               |
| **dev**   | `target/debug/codex`        | 1.28 G | `codex-cli 0.0.0`   | 动态                               |

冒烟测试（musl 与 glibc 各跑一遍）：`--version` / `--help` / `exec --help` / `mcp --help` / `code-mode-host --help` 全部通过。

构建耗时：musl 首次 **18m17s**；glibc **15m28s**。

### 5.2 脚本行为

| 项                | 命令                                                         | 结果                                                |
| ----------------- | ------------------------------------------------------------ | --------------------------------------------------- |
| V8 脚本从零可用   | `CODEX_V8_CACHE=/tmp/v8-test bash scripts/setup-rusty-v8.sh` | 下载 3 文件 + `verified checksums`                  |
| V8 幂等           | 第二次运行                                                   | 3 个文件全部命中缓存                                |
| **V8 篡改被拒**   | 改归档 1 字节后重跑                                          | 退出码 `1`，`artifact checksum verification failed` |
| **产物携带版本**  | `target/<t>/release/codex --version`                         | **`codex-cli 0.154.0`**（manifest 仍为 `0.0.0`）    |
| 工作区零残留      | 构建前后 `md5sum -c`                                         | `Cargo.toml: OK` / `Cargo.lock: OK`                 |
| 失败路径还原      | 传不存在的 target                                            | 退出码 `1`，manifest 仍 `md5 OK`                    |
| 中断路径还原      | `kill -INT` 脚本                                             | 还原执行，`md5 OK`                                  |
| **并发被拒**      | 同时起两个实例                                               | 第一个 `0`、第二个 `1` 且提示 `in progress`         |
| `--help` 任意 cwd | 从 `/tmp` 调用                                               | 正常输出                                            |
| dev 构建不受影响  | `target/debug/codex --version`                               | `codex-cli 0.0.0`                                   |
| 快照未被牵动      | 构建前后 `git status`                                        | 142 个 `.snap` 无改动                               |
| 快照与版本一致    | `just test -p codex-tui status_snapshot`                     | **28 passed / 0 failed**                            |
| `bwrap` 摘要自洽  | `grep -a <sha256> codex`                                     | 与当前 `bwrap` 的 sha256 一致                       |

## 六、踩坑记录（下次直接看这里）

五个都是**实跑才暴露**的，不是读代码能发现的。

### 6.1 `--help` 在 `cd` 之后失效

脚本 `cd` 到 `codex-rs/` 后仍用相对路径 `${BASH_SOURCE[0]}` 读自身 → `sed: can't read scripts/...`。
**修法**：`cd` 之前先算好绝对 `self_path`。

### 6.2 env 文件生成会执行注入

最初用 `grep ... | sed 's/^/export /'` 生成 env 文件。`CMAKE_ARGS`、`CFLAGS` 这类值**含空格**，会生成 `export CMAKE_ARGS=-DA -DB`，`source` 时 `-DB` 被当成命令。
**修法**：`printf 'export %s=%q\n'`，并用含空格的同形数据实测 `source` 后取值完整。

### 6.3 `zig` 装了但不在 PATH

只 `export PATH` 在脚本进程内，后续 shell 拿不到。
**修法**：软链到 `~/.local/bin/zig`（该目录本就在 PATH）。

### 6.4 `rustup target add` 装错 toolchain（musl 构建第一次失败的直接原因）

报错：`error[E0463]: can't find crate for 'core'`。

`rust-toolchain.toml` 在 **`codex-rs/`** 里，而 `rustup target add` 只对**当前 active toolchain** 生效。从仓库根跑 → 装进 `stable`(1.96.0)，但构建用 `1.95.0`：

```
仓库根:  stable (default)          → gnu, musl   ← 装错地方了
codex-rs: 1.95.0 (rust-toolchain)  → gnu         ← 构建实际用它
```

**修法**：`(cd "$repo_root/codex-rs" && rustup target add ...)`。

### 6.5 并发跑两个实例会把工作区搞脏（最隐蔽的一个）

实例 A 已把 manifest 改成注入版本，实例 B 此时启动 → B 把 **A 写入后的值**当成"原始备份"，退出时还原成它 → 工作区残留 `0.154.0`，而且两个实例都报成功。
**触发方式**：并发测试时意外踩到。**修法**：`mkdir` 锁（`flock` 在 macOS 上不存在），第二个实例直接拒绝退出。

### 6.6 附带的系统副作用（apt 装包引起）

装 musl 工具链那次 apt 事务改变了宿主环境，与 codex 无关但要知道：

- `/usr/bin/{clang,clang++,lld}` 从 `/opt/llvm-23` 变成 `/usr/lib/llvm-21` —— `clang` / `lld` 元包**直接用普通符号链接占用路径**，不参与 alternatives 优先级竞争（我事先预测"230 优先级最高不会切"是错的）。
  **回退**：`sudo apt-get remove -y clang lld && sudo update-alternatives --auto clang`（已用 `--altdir/--admindir` 影子目录实测 `--auto` 会检测 link group 损坏并强制重建 master + 全部 slave）。
- **`libc++-22-dev` / `libc++abi-22-dev` 被移除**（无版本 `libc++-dev` 元包只能指向一个版本，改 22 会卸掉 21，反之亦然）。
  **回退**：`sudo apt-get install -y libc++-22-dev libc++abi-22-dev`。
- `/opt/llvm-23`、`/opt/rocm` **本体零写入**（`find -newermt` 无结果；`clang-23` 时间戳仍为 06-11，md5 `6f133c2cf82199c15637763ec65f2cc0`）；ROCm 自己的 `amdclang` / `amdlld` / `rocm-llvm` alternatives 未被动过。

## 七、风险与边界

1. **构建期注入的边界**：
   - 它保证的是**构建产物**携带版本；工作区源码始终是 `0.0.0`，所以 `cargo test` 仍按 `0.0.0` 跑，快照天然一致。
   - **切换版本会触发全量重编**（实测 15～18 分钟）。不要计划"来回切换靠缓存"：脚本会把 `CODEX_BWRAP_SHA256` 内嵌进产物，而该摘要取决于注入的版本（`0.154.0` → `724ff4df…`，`0.0.0` → `8855806e…`）。实测：同一版本连续跑可命中缓存（20 秒）；版本变过之后即使切回去也可能全量重编。
   - 版本来源依赖仓库里存在 `rust-v*` tag；没有则回退到 manifest 现值并打印提示。
   - 构建期间工作区是"脏"的（几分钟到十几分钟）。mkdir 锁只阻止**本脚本**并发，不阻止你在此期间另开一个 `cargo test` —— 那个测试会看到被写入的版本。
2. **musl 已在 `x86_64-unknown-linux-musl` 上验证通过**；`aarch64-unknown-linux-musl` 未验证（脚本支持，但缺 ARM 机器）。
3. **`0.0.0` 会触发测试公告**：`announcement_tip.toml` 有 `version_regex = "^0\\.0\\.0$"` 的 "This is a test announcement" 条目，本地构建会显示。这是上游预期行为，不是缺陷。
4. **产物未 strip**：`[profile.release] strip = false`，1.2 GB 的 `codex` 是刻意保留符号以便打包前归档（见该 profile 注释），不是配置错误。
5. **CI 尚未落地**（§4.4）：目前是"本机能跑通"，不是"每次推送自动产出"。
