// This file is part of Moonfire NVR, a security camera digital video recorder.
// Copyright (C) 2017 Scott Lamb <slamb@slamb.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// In addition, as a special exception, the copyright holders give
// permission to link the code of portions of this program with the
// OpenSSL library under certain conditions as described in each
// individual source file, and distribute linked combinations including
// the two.
//
// You must obey the GNU General Public License in all respects for all
// of the code used other than OpenSSL. If you modify file(s) with this
// exception, you may extend this exception to your version of the
// file(s), but you are not obligated to do so. If you do not wish to do
// so, delete this exception statement from your version. If you delete
// this exception statement from all source files in the program, then
// also delete it here.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate gcc;
extern crate pkg_config;

fn main() {
    let libraries = [
        pkg_config::Config::new().atleast_version("54.1").probe("libavutil").unwrap(),
        pkg_config::Config::new().atleast_version("56.0").probe("libavcodec").unwrap(),
        pkg_config::Config::new().atleast_version("56.0").probe("libavformat").unwrap(),
    ];
    let mut wrapper = gcc::Config::new();

    // Pass compilation flags on to gcc. It'd be nice if pkg-config made this easier; see
    // <https://github.com/alexcrichton/pkg-config-rs/issues/43>.
    for lib in &libraries {
        for p in &lib.include_paths {
            wrapper.include(p);
        }
        for l in &lib.libs {
            println!("lib: {}", l);
        }
    }

    wrapper.file("wrapper.c").compile("libwrapper.a");
}