<p align="center">
  <em>Screenshot coming soon — single window, checkbox list per category, Apply / Remove buttons, live stats in the header.</em>
</p>

<p align="center">
  <a href="https://github.com/namefailed/chaff/actions"><img src="https://github.com/namefailed/chaff/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
  <a href="https://github.com/namefailed/chaff/releases"><img src="https://img.shields.io/github/downloads/namefailed/chaff/total" alt="Downloads"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue" alt="License"></a>
</p>

# 🪶 Chaff

**Make your machine look like a malware analyst's sandbox.**

Click Apply. Go about your day.

Chaff plants the exact breadcrumbs ransomware and info-stealers check before deploying — fake registry keys, named mutexes, open pipes, and ghost processes running under debugger and AV names. Most commodity malware sees them and quietly exits. **Without ever being detected or blocked.**

Everything is cleaned up the moment you hit Remove. No residue, no installer, no background service.

---

## 🧠 Philosophy

| Principle | What It Means |
|-----------|---------------|
| **🎭 Deception over detection** | Chaff doesn't block or scan anything. It makes malware decide on its own that your machine isn't worth touching. Nothing to update, nothing to bypass. |
| **🪶 Zero footprint** | No service, no driver, no installer. One `.exe`. Everything Chaff creates — registry keys, handles, ghost processes — vanishes when you click Remove or close the app. |
| **🎲 Randomized by default** | Process categories sample a random subset each run. The same malware that fingerprinted your process list last week will see a different one today. |

---

## 🎯 Why deception works

Malware doesn't want to be analyzed. Before executing, most commodity samples do a fast environment check — is a debugger running? Is this a VM? Does Wireshark exist in the process list? If the answer is yes, the sample aborts. Not because it was caught, but because it chose to leave.

Chaff exploits that logic:

- **Home users** — most ransomware and stealers aren't targeted. They're spray-and-pray. Anything that looks like an analyst box gets skipped automatically.
- **Researchers and red teamers** — test samples in a live environment without spinning up a full VM. Ghost processes and registry keys are instant; tear them down after.
- **Environments without EDR** — deception artifacts cost nothing to maintain and require no definitions, no cloud, no subscription.
- **Defense in depth** — even with AV installed, a sample that self-terminates on entry never gets a chance to exploit a zero-day or misconfiguration.

Chaff doesn't protect against targeted attacks or sophisticated malware that validates its environment more carefully. It's a speed bump for the vast majority of commodity threats that use static checklists.

---

## ⚙️ How it works

| Artifact type | What Chaff does | Cleaned up by |
|---|---|---|
| **Registry key** | Creates the key path under `HKLM\` or `HKCU\` — the path itself is the signal, no values needed | Remove button |
| **Mutex** | Calls `CreateMutexW` and holds the handle open for the lifetime of the app | Remove button (handle closed on drop) |
| **Named pipe** | Calls `CreateNamedPipeW` and holds the server end open | Remove button (handle closed on drop) |
| **Ghost process** | Copies itself to `%TEMP%\chaff\<name>.exe` and launches it with `--ghost`; the copy sleeps forever and shows up in Task Manager under the fake name | Remove button (process killed, temp copy deleted) |

> **`HKLM\` keys require admin rights.** Right-click Chaff → Run as administrator to plant VM and antivirus registry artifacts. `HKCU\` keys, mutexes, pipes, and ghost processes all work without elevation.

> **⚠️ Force-kill leaves registry keys behind.** Clicking Remove or using Quit from the tray menu cleans up everything — registry keys, handles, ghost processes, and `%TEMP%\chaff\`. If the process is hard-killed (Task Manager → End Process, power loss, crash), mutexes and pipes are released by the OS automatically, but registry keys persist until you run Chaff again and click Remove. This is unavoidable without a kernel driver. Use End Task (not End Process) if you need to kill Chaff from Task Manager.

---

## ✨ Features

- **✅ Category checkboxes** — enable only what you want. VM registry artifacts, AV keys, analysis tool keys, sandbox indicators, mutexes, pipes, and six process pools are all independent.
- **🎲 Random process sampling** — each process category has a pool of 12–64 names and a sample size. Chaff picks a random subset every time you Apply, so the process list varies across runs.
- **🔢 Process count slider** — override the per-category sample with a global slider (1–20 processes per category).
- **📊 Live stats** — the header shows active process count, registry key count, and open handle count in real time.
- **🖥️ System tray** — minimize to tray; Open / Quit from the context menu. The tray tooltip updates with active counts.
- **📋 Log file** — every Apply and Remove action is written to `%APPDATA%\chaff\chaff.log` with a Unix timestamp.
- **⚡ Single binary** — no installer, no runtime, no dependencies. Drop the `.exe` anywhere and run it.

---

## 🛡️ Artifact catalog

<details>
<summary><strong>VM Registry Keys</strong> — VirtualBox (21), VMware (14), Hyper-V (11), QEMU (9)</summary>

**VirtualBox**

| Key |
|---|
| `HKLM\SOFTWARE\Oracle\VirtualBox` |
| `HKLM\SOFTWARE\Oracle\VirtualBox Guest Additions` |
| `HKLM\HARDWARE\ACPI\DSDT\VBOX__` |
| `HKLM\HARDWARE\ACPI\FADT\VBOX__` |
| `HKLM\HARDWARE\ACPI\RSDT\VBOX__` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxGuest` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxMouse` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxSF` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxService` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxVideo` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxNetAdp` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxNetFlt` |
| `HKLM\SYSTEM\ControlSet001\Services\VBoxUSB` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxGuest` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxMouse` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxSF` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxService` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxVideo` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxNetAdp` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxNetFlt` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VBoxUSB` |

**VMware**

| Key |
|---|
| `HKLM\SOFTWARE\VMware, Inc.` |
| `HKLM\SOFTWARE\VMware, Inc.\VMware Tools` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmware` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmvss` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMTools` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMMEMCTL` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMUSB` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmci` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmx86` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMware NAT Service` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMnetAdapter` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMnetBridge` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmnetuserif` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicheartbeat` |

**Hyper-V**

| Key |
|---|
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicvss` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicshutdown` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicexchange` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicguestinterface` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vmicvmsession` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMBusHID` |
| `HKLM\SYSTEM\CurrentControlSet\Services\VMBusHIDMonitor` |
| `HKLM\SYSTEM\CurrentControlSet\Services\storvsp` |
| `HKLM\SYSTEM\CurrentControlSet\Services\storflt` |
| `HKLM\SYSTEM\CurrentControlSet\Services\storvsc` |
| `HKLM\SYSTEM\CurrentControlSet\Services\vid` |

**QEMU**

| Key |
|---|
| `HKLM\HARDWARE\ACPI\DSDT\QEMU` |
| `HKLM\HARDWARE\ACPI\FADT\QEMU` |
| `HKLM\HARDWARE\ACPI\RSDT\QEMU` |
| `HKLM\SYSTEM\CurrentControlSet\Services\QEMU` |
| `HKLM\SYSTEM\CurrentControlSet\Services\qemupciserial` |
| `HKLM\SYSTEM\CurrentControlSet\Services\qemudisk` |
| `HKLM\SYSTEM\CurrentControlSet\Services\qemuprocessemu` |
| `HKLM\SYSTEM\CurrentControlSet\Services\qemuserial` |
| `HKLM\SYSTEM\CurrentControlSet\Services\qemuvideo` |

</details>

<details>
<summary><strong>Antivirus Registry Keys</strong> — 10 vendors</summary>

| Key |
|---|
| `HKLM\SOFTWARE\Malwarebytes` |
| `HKLM\SOFTWARE\ESET` |
| `HKLM\SOFTWARE\Avast Software` |
| `HKLM\SOFTWARE\AVG` |
| `HKLM\SOFTWARE\Kaspersky Lab` |
| `HKLM\SOFTWARE\Bitdefender` |
| `HKLM\SOFTWARE\McAfee` |
| `HKLM\SOFTWARE\Symantec` |
| `HKLM\SOFTWARE\Sophos` |
| `HKLM\SOFTWARE\TrendMicro` |

</details>

<details>
<summary><strong>Analysis Tool Registry Keys</strong> — Wireshark + Sysinternals (7)</summary>

| Key |
|---|
| `HKLM\SOFTWARE\Wireshark` |
| `HKCU\SOFTWARE\Sysinternals` |
| `HKCU\SOFTWARE\Sysinternals\Process Monitor` |
| `HKCU\SOFTWARE\Sysinternals\Process Explorer` |
| `HKCU\SOFTWARE\Sysinternals\Autoruns` |
| `HKCU\SOFTWARE\Sysinternals\Strings` |
| `HKCU\SOFTWARE\Sysinternals\ProcDump` |

</details>

<details>
<summary><strong>Sandbox Registry Keys</strong> — Cuckoo + Sandboxie (4)</summary>

| Key |
|---|
| `HKLM\SOFTWARE\Cuckoo` |
| `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Sandboxie` |
| `HKLM\SYSTEM\ControlSet001\Services\SbieDrv` |
| `HKLM\SYSTEM\ControlSet001\Services\SandboxieRpcSs` |

</details>

<details>
<summary><strong>Named Mutexes</strong> — 6, held open while Chaff runs</summary>

| Mutex |
|---|
| `PROCMON_PORT_MUTEX` |
| `Wireshark-is-running-{9508AFC4-9B86-498E-8F08-59E3F1EFAB7A}` |
| `MicrosoftMalwareProtectionRemoteIoRequest` |
| `ProcExpMutex` |
| `cuckoo_agent_ctrl` |
| `_SHuassist` |

</details>

<details>
<summary><strong>VM Named Pipes</strong> — 3, server end held open while Chaff runs</summary>

| Pipe |
|---|
| `\\.\pipe\vmware_vgauth` |
| `\\.\pipe\VMWareClient` |
| `\\.\pipe\VBoxTrayIPC` |

</details>

<details>
<summary><strong>Debugger Processes</strong> — 22 names, 5 sampled per run</summary>

`ollydbg.exe` · `x64dbg.exe` · `windbg.exe` · `immunitydebugger.exe` · `gdb.exe` · `radare2.exe` · `ida.exe` · `ida64.exe` · `softice.exe` · `d4l.exe` · `hiew.exe` · `dbgview.exe` · `debugview.exe` · `syser.exe` · `w32dasm.exe` · `fdbg.exe` · `ollydbg64.exe` · `debugger.exe` · `ollyice.exe` · `megadbg.exe` · `sicedt.exe` · `wdmkit.exe`

</details>

<details>
<summary><strong>Decompiler Processes</strong> — 12 names, 3 sampled per run</summary>

`hexraysdecompiler.exe` · `ghidra.exe` · `retdec.exe` · `bochs.exe` · `titanengine.exe` · `javadecompiler.exe` · `dnspy.exe` · `ilspy.exe` · `dotpeek.exe` · `procyon.exe` · `snowman.exe` · `frida.exe`

</details>

<details>
<summary><strong>VM Host Processes</strong> — 20 names, 5 sampled per run</summary>

`vmware.exe` · `vmware-vmx.exe` · `vmwareuser.exe` · `vmwareservice.exe` · `vboxservice.exe` · `vboxtray.exe` · `virtualbox.exe` · `vboxheadless.exe` · `parallels.exe` · `qemu.exe` · `vagrant.exe` · `vmusrvc.exe` · `vmtoolsd.exe` · `vmsrvc.exe` · `vmwaretray.exe` · `vboxcontrol.exe` · `vbox.exe` · `vboxsdl.exe` · `vboxwebsrv.exe` · `parallelsvm.exe`

</details>

<details>
<summary><strong>Sandbox Processes</strong> — 15 names, 3 sampled per run</summary>

`cuckoo.exe` · `sandboxie.exe` · `comodosandbox.exe` · `detours.exe` · `anubis.exe` · `gfi.exe` · `joeboxcontrol.exe` · `safescanner.exe` · `bsa.exe` · `threatanalyzer.exe` · `shadowboxer.exe` · `fireeyetool.exe` · `malwarehost.exe` · `firelamb.exe` · `vas.exe`

</details>

<details>
<summary><strong>System Monitoring Processes</strong> — 23 names, 5 sampled per run</summary>

`procmon.exe` · `procexp.exe` · `regmon.exe` · `filemon.exe` · `wireshark.exe` · `fiddler.exe` · `tcpview.exe` · `autoruns.exe` · `apimonitor.exe` · `processhacker.exe` · `sysinspector.exe` · `regrunreanimator.exe` · `securitytaskmanager.exe` · `netmon.exe` · `ethereal.exe` · `spythemall.exe` · `processexplorer.exe` · `taskcatcher.exe` · `processrevealer.exe` · `procanalyzer.exe` · `resmon.exe` · `netviewer.exe` · `scylla.exe`

</details>

<details>
<summary><strong>Antivirus Processes</strong> — 64 names, 5 sampled per run</summary>

`avp.exe` · `avgnt.exe` · `ahnsd.exe` · `bdss.exe` · `bdagent.exe` · `egui.exe` · `ekrn.exe` · `avguard.exe` · `ccavsrv.exe` · `clamtray.exe` · `clamscan.exe` · `msmpeng.exe` · `mssense.exe` · `savservice.exe` · `saswinlo.exe` · `sbamtray.exe` · `spbbcsvc.exe` · `wrsa.exe` · `zlclient.exe` · `avastui.exe` · `ashdisp.exe` · `avastsvc.exe` · `avgui.exe` · `avgsvca.exe` · `fsdfwd.exe` · `vsserv.exe` · `mfemms.exe` · `mfevtps.exe` · `mcshield.exe` · `rtvscan.exe` · `navapsvc.exe` · `navw32.exe` · `ccapp.exe` · `drweb.exe` · `drwebd.exe` · `spideragent.exe` · `fortitray.exe` · `fortiscanner.exe` · `fortiedr.exe` · `pccntmon.exe` · `tmproxy.exe` · `tmntsrv.exe` · `mbam.exe` · `mbamservice.exe` · `mbamtray.exe` · `msascui.exe` · `msascuil.exe` · `msseces.exe` · `psafe.exe` · `df5serv.exe` · `dssagent.exe` · `antivirus.exe` · `dwengine.exe` · `dwscan.exe` · `cmdagent.exe` · `cis.exe` · `cfp.exe` · `cavwp.exe` · `avcenter.exe` · `avgsvc.exe` · `avshadow.exe` · `spiderml.exe` · `drwebupw.exe`

</details>

---

## 🫡 Prior art

Chaff wouldn't exist without these:

- **[CyberScarecrow](https://www.cyberscarecrow.com/)** — the commercial product that popularized this approach for home users. Closed source, Windows only, free. Chaff is the open-source answer to it.
- **[Malcrow](https://github.com/joaovarelas/Malcrow)** — an open-source malware deception tool that proved the concept works without commercial software. Unmaintained since 2022, but the ideas here are a direct evolution of that work.

---

## 🚀 Quick start

**Download** the latest `chaff.exe` from the [Releases](https://github.com/namefailed/chaff/releases) page. The release binary has the full artifact list baked in — no setup, no installer.

Right-click → **Run as administrator** to plant `HKLM\` registry keys. Standard user is fine for everything else.

Click **▶ Apply** to plant artifacts. Click **■ Remove** to tear everything down. The system tray keeps Chaff alive when you close the window — use **Quit** from the tray menu to exit completely.

**Build from source**

```powershell
git clone https://github.com/namefailed/chaff.git
cd chaff
cargo build --release
# Binary at target\release\chaff.exe
```

> The artifact database is kept in a private repository and baked into release binaries at build time. If you build from source without it, `build.rs` generates a minimal stub so the code compiles and runs — you just won't get the full indicator list. Download a release binary if you want the complete set.

---

## 📚 Documentation

| Topic | Where |
|---|---|
| Full artifact list | Scroll up — everything is listed in the catalog above |
| Log file | `%APPDATA%\chaff\chaff.log` |
| Ghost process temp dir | `%TEMP%\chaff\` — cleared on Remove |

---

## 🧱 Tech stack

| Layer | Choice |
|---|---|
| **UI** | [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) — immediate-mode GUI, single native binary, no webview |
| **Registry** | [winreg](https://github.com/gentoo90/winreg-rs) |
| **Mutexes / pipes** | [winapi](https://github.com/retep998/winapi-rs) — `CreateMutexW`, `CreateNamedPipeW` |
| **System tray** | [tray-icon](https://github.com/tauri-apps/tray-icon) |
| **Random sampling** | [rand](https://github.com/rust-random/rand) — process categories pick a random subset each run |

Single `chaff.exe`. No installer, no runtime, no dependencies.

---

## 🤝 Contributing

Bug reports and pull requests are welcome. Open an issue first if you're proposing something bigger.

---

## 📄 License

MIT OR Apache-2.0.

Built by [@namefailed](https://github.com/namefailed). No accounts, no telemetry, no tracking.
