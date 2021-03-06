// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
extern crate advapi32;
extern crate test;
use advapi32::*;
use test::black_box as bb;
#[test]
fn functions() {
    bb(AdjustTokenPrivileges);
    bb(CloseServiceHandle);
    bb(ControlService);
    bb(CreateServiceA);
    bb(CreateServiceW);
    bb(CryptAcquireContextA);
    bb(CryptAcquireContextW);
    bb(CryptCreateHash);
    bb(CryptDestroyHash);
    bb(CryptGetHashParam);
    bb(CryptHashData);
    bb(CryptReleaseContext);
    bb(DeleteService);
    bb(OpenProcessToken);
    bb(OpenSCManagerA);
    bb(OpenSCManagerW);
    bb(OpenServiceA);
    bb(OpenServiceW);
    bb(QueryServiceStatus);
    bb(QueryServiceStatusEx);
    bb(RegCloseKey);
    bb(RegConnectRegistryA);
    bb(RegConnectRegistryW);
    bb(RegCopyTreeA);
    bb(RegCopyTreeW);
    bb(RegCreateKeyExA);
    bb(RegCreateKeyExW);
    bb(RegDeleteKeyA);
    bb(RegDeleteKeyExA);
    bb(RegDeleteKeyExW);
    bb(RegDeleteKeyValueA);
    bb(RegDeleteKeyValueW);
    bb(RegDeleteKeyW);
    bb(RegDeleteTreeA);
    bb(RegDeleteTreeW);
    bb(RegDeleteValueA);
    bb(RegDeleteValueW);
    bb(RegDisablePredefinedCache);
    bb(RegDisablePredefinedCacheEx);
    bb(RegDisableReflectionKey);
    bb(RegEnableReflectionKey);
    bb(RegEnumKeyExA);
    bb(RegEnumKeyExW);
    bb(RegEnumValueA);
    bb(RegEnumValueW);
    bb(RegFlushKey);
    bb(RegGetValueA);
    bb(RegGetValueW);
    bb(RegLoadMUIStringW);
    bb(RegNotifyChangeKeyValue);
    bb(RegOpenCurrentUser);
    bb(RegOpenKeyExA);
    bb(RegOpenKeyExW);
    bb(RegOpenUserClassesRoot);
    bb(RegOverridePredefKey);
    bb(RegQueryInfoKeyA);
    bb(RegQueryInfoKeyW);
    bb(RegQueryMultipleValuesA);
    bb(RegQueryMultipleValuesW);
    bb(RegQueryReflectionKey);
    bb(RegQueryValueExA);
    bb(RegQueryValueExW);
    bb(RegSetKeyValueA);
    bb(RegSetValueExA);
    bb(RegSetValueExW);
    bb(RegSetKeyValueW);
    bb(RegisterServiceCtrlHandlerA);
    bb(RegisterServiceCtrlHandlerExA);
    bb(RegisterServiceCtrlHandlerExW);
    bb(RegisterServiceCtrlHandlerW);
    bb(SetServiceStatus);
    bb(StartServiceCtrlDispatcherA);
    bb(StartServiceCtrlDispatcherW);
}
