#![allow(warnings)]
// Bitcoin secp256k1 bindings
// Written in 2015 by
//   Andrew Poelstra
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Build script

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate cc;

use std::env;

fn main() {
    if cfg!(feature = "external-symbols") {
        println!("cargo:rustc-link-lib=static=secp256k1zkp");
        return;
    }

    // Actual build
    let mut base_config = cc::Build::new();
    base_config
        .include("depend/secp256k1/")
        .include("depend/secp256k1/include")
        .include("depend/secp256k1/src")
        .flag_if_supported("-Wno-unused-function") // some ecmult stuff is defined but not used upstream
        .define("SECP256K1_BUILD", Some(""))
        .define("ENABLE_MODULE_SURJECTIONPROOF", Some("1"))
        .define("ENABLE_MODULE_GENERATOR", Some("1"))
        .define("ENABLE_MODULE_RANGEPROOF", Some("1"))
        .define("ENABLE_MODULE_ECDSA_ADAPTOR", Some("1"))
        .define("ENABLE_MODULE_WHITELIST", Some("1"))
        .define("ENABLE_MODULE_EXTRAKEYS", Some("1"))
        .define("ENABLE_MODULE_MUSIG", Some("1"))
        .define("ENABLE_MODULE_SCHNORRSIG", Some("1"))
        .define("ECMULT_GEN_PREC_BITS", Some("4"))
        // TODO these three should be changed to use libgmp, at least until secp PR 290 is merged
        .define("USE_NUM_NONE", Some("1"))
        .define("USE_FIELD_INV_BUILTIN", Some("1"))
        .define("USE_SCALAR_INV_BUILTIN", Some("1"));

    if cfg!(feature = "lowmemory") {
        base_config.define("ECMULT_WINDOW_SIZE", Some("4")); // A low-enough value to consume neglible memory
    } else {
        base_config.define("ECMULT_WINDOW_SIZE", Some("15")); // This is the default in the configure file (`auto`)
    }
    base_config.define("USE_EXTERNAL_DEFAULT_CALLBACKS", Some("1"));

    if let Ok(target_endian) = env::var("CARGO_CFG_TARGET_ENDIAN") {
        if target_endian == "big" {
            base_config.define("WORDS_BIGENDIAN", Some("1"));
        }
    }

    // Header files. WASM only.
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        base_config.include("wasm-sysroot");
    }

    // secp256k1
    base_config
        .file("depend/secp256k1/contrib/lax_der_parsing.c")
        .file("depend/secp256k1/src/secp256k1.c")
        .file("depend/secp256k1/src/precomputed_ecmult_gen.c")
        .file("depend/secp256k1/src/precomputed_ecmult.c")
        .compile("libsecp256k1zkp.a");
}
