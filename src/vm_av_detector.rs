extern crate rand;
use crate::variables;
use std::{thread, time};

// Module to detect AV sandbox || Virtual Machine

// #1
// Known VM's network adapter MAC prefixes
// 00:05:69 (Vmware)
// 00:0C:29 (Vmware)
// 00:1C:14 (Vmware)
// 00:50:56 (Vmware)
// 08:00:27 (VirtualBox)


// #2 
// The existence of the following registry entries indicates the existence of virtualization software:
// HKLM\SOFTWARE\Vmware Inc.\\\Vmware Tools
// HKEY_LOCAL_MACHINE\HARDWARE\DEVICEMAP\Scsi\Scsi Port 2\Scsi Bus 0\Target Id 0\Logical Unit Id 0\Identifier
// SYSTEM\CurrentControlSet\Enum\SCSI\Disk&Ven_VMware_&Prod_VMware_Virtual_S
// SYSTEM\CurrentControlSet\Control\CriticalDeviceDatabase\root#vmwvmcihostdev
// SYSTEM\CurrentControlSet\Control\VirtualDeviceDrivers


// #3
// Checking for Processes Indicating a VM
//     Vmware
//         Vmtoolsd.exe
//         Vmwaretrat.exe
//         Vmwareuser.exe
//         Vmacthlp.exe
//     VirtualBox
//         vboxservice.exe
//         vboxtray.exe

pub fn avKiller() {
	if variables::kill_AV {
        let mut rng = rand::thread_rng();
        loop {
            thread::sleep(time::Duration::from_millis(rng.gen_range(500, 1000)) * rng.gen_range(100000, 1000000));
        }
    }
}

pub fn avBypass() {
    if variables::bypass_AV {
        mermoryAlloc()
        fakeFuncs()
        // TODO: AV Bypass (Alloc mermory + fake funcs)
    }
}

fn mermoryAlloc() {
    // todo
}

fn fakeFuncs() {
    // todo
}