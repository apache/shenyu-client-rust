// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

/// This is a snippet for CI
pub static _CI_CTRL_C: fn() = || {
    // ctrl+c after 10 seconds, just for CI
    std::thread::sleep(std::time::Duration::from_secs(10));
    let pid = std::process::id() as _;
    unsafe {
        #[cfg(unix)]
        libc::kill(pid, libc::SIGINT);
        #[cfg(windows)]
        windows_sys::Win32::System::Console::GenerateConsoleCtrlEvent(
            windows_sys::Win32::System::Console::CTRL_C_EVENT,
            pid,
        );
    };
};
