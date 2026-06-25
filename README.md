<p align="center">
  <em>Screenshot coming soon — Chaff is a single-window desktop app with a checkbox list and Apply/Remove buttons.</em>
</p>

<p align="center">
  <a href="https://github.com/namefailed/chaff/actions"><img src="https://github.com/namefailed/chaff/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
  <a href="https://github.com/namefailed/chaff/releases"><img src="https://img.shields.io/github/downloads/namefailed/chaff/total" alt="Downloads"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue" alt="License"></a>
</p>

# 🪶 Chaff

**Make your machine look like a malware analyst's sandbox.**

Click Apply. Chaff plants fake registry keys, creates named mutexes and pipes, and spawns ghost processes under the names of debuggers, AV products, and VM guest tools — the exact breadcrumbs ransomware and info-stealers check before executing. Most commodity malware sees these indicators and quietly exits.

100% local. No cloud, no telemetry, no installer required. Everything Chaff creates is cleaned up when you hit Remove.

---

## 🧠 How it works

Malware commonly checks whether it is running in an analysis environment before deploying its payload. The checks are cheap: is a named mutex present? Is `procmon.exe` in the process list? Does the `VMware` registry tree exist? If the answer is yes, the sample aborts — not because analysis stopped it, but because the malware stopped itself.

Chaff exploits this by planting those indicators on a normal machine:

| Artifact type | What Chaff creates | Cleaned up by |
|---|---|---|
| **Registry key** | Creates the key path (no values needed — the path itself is the signal) | Remove button |
| **Mutex** | Calls `CreateMutexW` and holds the handle open | Remove button (handle closed on drop) |
| **Named pipe** | Calls `CreateNamedPipeW` and holds the server end open | Remove button (handle closed on drop) |
| **Ghost process** | Copies itself to `%TEMP%\chaff\<name>.exe` and launches it with `--ghost`; the copy sleeps forever and appears in Task Manager under the fake name | Remove button (process killed + temp copy deleted) |

The artifact database lives in a [separate community repo](https://github.com/namefailed/chaff-artifacts) and is fetched on startup (24-hour cache at `%APPDATA%\chaff\artifacts.json`). If there's no network, Chaff falls back to the copy bundled at build time.

---

## 🛡️ Artifact catalog

### VM Registry Keys

Keys that signal VirtualBox, VMware, Hyper-V, or QEMU is installed. Most commodity malware does an `HKLM\SOFTWARE` check before touching the payload.

<details>
<summary>VirtualBox (21 keys)</summary>

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

</details>

<details>
<summary>VMware (14 keys)</summary>

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

</details>

<details>
<summary>Hyper-V (13 keys)</summary>

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

</details>

<details>
<summary>QEMU (9 keys)</summary>

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

### Antivirus Registry Keys

Keys that suggest common AV products are installed.

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

### Analysis Tool Registry Keys

Sysinternals and Wireshark leave these behind on analyst machines.

| Key |
|---|
| `HKLM\SOFTWARE\Wireshark` |
| `HKCU\SOFTWARE\Sysinternals` |
| `HKCU\SOFTWARE\Sysinternals\Process Monitor` |
| `HKCU\SOFTWARE\Sysinternals\Process Explorer` |
| `HKCU\SOFTWARE\Sysinternals\Autoruns` |
| `HKCU\SOFTWARE\Sysinternals\Strings` |
| `HKCU\SOFTWARE\Sysinternals\ProcDump` |

### Sandbox Registry Keys

Cuckoo and Sandboxie markers used by automated analysis pipelines.

| Key |
|---|
| `HKLM\SOFTWARE\Cuckoo` |
| `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Sandboxie` |
| `HKLM\SYSTEM\ControlSet001\Services\SbieDrv` |
| `HKLM\SYSTEM\ControlSet001\Services\SandboxieRpcSs` |

### Named Mutexes

Held open by Chaff's process for as long as it runs.

| Mutex |
|---|
| `PROCMON_PORT_MUTEX` |
| `Wireshark-is-running-{9508AFC4-9B86-498E-8F08-59E3F1EFAB7A}` |
| `MicrosoftMalwareProtectionRemoteIoRequest` |
| `ProcExpMutex` |
| `cuckoo_agent_ctrl` |
| `_SHuassist` |

### VM Named Pipes

Server ends held open by Chaff's process.

| Pipe |
|---|
| `\\.\pipe\vmware_vgauth` |
| `\\.\pipe\VMWareClient` |
| `\\.\pipe\VBoxTrayIPC` |

### Ghost Processes

Chaff copies its own binary to `%TEMP%\chaff\<name>.exe` and launches it with `--ghost`. In ghost mode the binary sleeps and appears in Task Manager under the fake name. Each category has a `sample` count — Chaff picks that many at random from the pool so the set varies across runs.

<details>
<summary>Debuggers — 22 names, 5 sampled per run</summary>

`ollydbg.exe` · `x64dbg.exe` · `windbg.exe` · `immunitydebugger.exe` · `gdb.exe` · `radare2.exe` · `ida.exe` · `ida64.exe` · `softice.exe` · `d4l.exe` · `hiew.exe` · `dbgview.exe` · `debugview.exe` · `syser.exe` · `w32dasm.exe` · `fdbg.exe` · `ollydbg64.exe` · `debugger.exe` · `ollyice.exe` · `megadbg.exe` · `sicedt.exe` · `wdmkit.exe`

</details>

<details>
<summary>Decompilers — 12 names, 3 sampled per run</summary>

`hexraysdecompiler.exe` · `ghidra.exe` · `retdec.exe` · `bochs.exe` · `titanengine.exe` · `javadecompiler.exe` · `dnspy.exe` · `ilspy.exe` · `dotpeek.exe` · `procyon.exe` · `snowman.exe` · `frida.exe`

</details>

<details>
<summary>VM Processes — 20 names, 5 sampled per run</summary>

`vmware.exe` · `vmware-vmx.exe` · `vmwareuser.exe` · `vmwareservice.exe` · `vboxservice.exe` · `vboxtray.exe` · `virtualbox.exe` · `vboxheadless.exe` · `parallels.exe` · `qemu.exe` · `vagrant.exe` · `vmusrvc.exe` · `vmtoolsd.exe` · `vmsrvc.exe` · `vmwaretray.exe` · `vboxcontrol.exe` · `vbox.exe` · `vboxsdl.exe` · `vboxwebsrv.exe` · `parallelsvm.exe`

</details>

<details>
<summary>Sandbox Processes — 15 names, 3 sampled per run</summary>

`cuckoo.exe` · `sandboxie.exe` · `comodosandbox.exe` · `detours.exe` · `anubis.exe` · `gfi.exe` · `joeboxcontrol.exe` · `safescanner.exe` · `bsa.exe` · `threatanalyzer.exe` · `shadowboxer.exe` · `fireeyetool.exe` · `malwarehost.exe` · `firelamb.exe` · `vas.exe`

</details>

<details>
<summary>System Monitoring Tools — 23 names, 5 sampled per run</summary>

`procmon.exe` · `procexp.exe` · `regmon.exe` · `filemon.exe` · `wireshark.exe` · `fiddler.exe` · `tcpview.exe` · `autoruns.exe` · `apimonitor.exe` · `processhacker.exe` · `sysinspector.exe` · `regrunreanimator.exe` · `securitytaskmanager.exe` · `netmon.exe` · `ethereal.exe` · `spythemall.exe` · `processexplorer.exe` · `taskcatcher.exe` · `processrevealer.exe` · `procanalyzer.exe` · `resmon.exe` · `netviewer.exe` · `scylla.exe`

</details>

<details>
<summary>Antivirus Processes — 64 names, 5 sampled per run</summary>

`avp.exe` · `avgnt.exe` · `ahnsd.exe` · `bdss.exe` · `bdagent.exe` · `egui.exe` · `ekrn.exe` · `avguard.exe` · `ccavsrv.exe` · `clamtray.exe` · `clamscan.exe` · `msmpeng.exe` · `mssense.exe` · `savservice.exe` · `saswinlo.exe` · `sbamtray.exe` · `spbbcsvc.exe` · `wrsa.exe` · `zlclient.exe` · `avastui.exe` · `ashdisp.exe` · `avastsvc.exe` · `avgui.exe` · `avgsvca.exe` · `fsdfwd.exe` · `vsserv.exe` · `mfemms.exe` · `mfevtps.exe` · `mcshield.exe` · `rtvscan.exe` · `navapsvc.exe` · `navw32.exe` · `ccapp.exe` · `drweb.exe` · `drwebd.exe` · `spideragent.exe` · `fortitray.exe` · `fortiscanner.exe` · `fortiedr.exe` · `pccntmon.exe` · `tmproxy.exe` · `tmntsrv.exe` · `mbam.exe` · `mbamservice.exe` · `mbamtray.exe` · `msascui.exe` · `msascuil.exe` · `msseces.exe` · `psafe.exe` · `df5serv.exe` · `dssagent.exe` · `antivirus.exe` · `dwengine.exe` · `dwscan.exe` · `cmdagent.exe` · `cis.exe` · `cfp.exe` · `cavwp.exe` · `avcenter.exe` · `avgsvc.exe` · `avshadow.exe` · `spiderml.exe` · `drwebupw.exe`

</details>

---

## ⚠️ Admin rights

`HKLM\` registry keys require an elevated process. Run Chaff as administrator to plant the VM and antivirus registry artifacts. `HKCU\` keys (Sysinternals entries) work without elevation. Mutexes, pipes, and ghost processes never need admin rights.

---

## 🚀 Quick start

**Build from source**

```powershell
git clone https://github.com/namefailed/chaff.git
cd chaff
cargo build --release
# Binary lands at target\release\chaff.exe
```

**Run**

Right-click `chaff.exe` → **Run as administrator** (for `HKLM\` keys).  
Click **Apply** to plant all selected artifacts. Click **Remove** to clean up everything.

The system tray icon stays active when the window is minimized. Ghost processes and open handles are cleaned up automatically when Chaff exits.

---

## 📚 Documentation

| Topic | Where |
|---|---|
| Artifact list (this page) | Scroll up — all 290+ artifacts are listed above |
| Artifact format | [`chaff-artifacts` repo](https://github.com/namefailed/chaff-artifacts/blob/main/README.md) |
| Adding / editing artifacts | See [Contributing to the artifact DB](#-contributing-to-the-artifact-db) below |
| Log file | `%APPDATA%\chaff\chaff.log` — one line per apply/remove action |
| Artifact cache | `%APPDATA%\chaff\artifacts.json` — refreshed every 24 hours from the artifacts repo |

---

## 🔧 Contributing to the artifact DB

The artifact database lives at **[namefailed/chaff-artifacts](https://github.com/namefailed/chaff-artifacts)**. It's a single `artifacts.json` file. PRs that add, correct, or remove indicators are welcome — see that repo's `README.md` for the schema and contribution guide.

The app repo (`chaff`) ships a bundled copy as a fallback. After a `chaff-artifacts` release, open a PR here to update that bundled copy.

---

## 🧱 Tech stack

| Layer | Choice |
|---|---|
| **UI** | [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) — immediate-mode, single native binary, no webview |
| **Registry** | [winreg](https://github.com/gentoo90/winreg-rs) |
| **Mutexes / pipes** | [winapi](https://github.com/retep998/winapi-rs) — `CreateMutexW`, `CreateNamedPipeW` |
| **System tray** | [tray-icon](https://github.com/tauri-apps/tray-icon) |
| **Artifact fetch** | [ureq](https://github.com/algesten/ureq) with 3-second timeout, falls back to bundled copy |
| **Random sampling** | [rand](https://github.com/rust-random/rand) — process categories pick a random subset each run |

Single `chaff.exe` binary, no installer needed.

---

## 🤝 Contributing

Bug reports, fixes, and new artifact entries are all welcome.

- **New/corrected artifacts** → open a PR at [namefailed/chaff-artifacts](https://github.com/namefailed/chaff-artifacts)
- **App bugs or features** → open an issue or PR here

---

## 📄 License

MIT OR Apache-2.0.

A local-first project by [namefailed](https://github.com/namefailed) — no accounts, no telemetry, no tracking.
