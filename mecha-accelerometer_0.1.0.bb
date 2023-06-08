# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get mecha-accelerometer could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/mecha-accelerometer/0.1.0"
SRC_URI += "git://github.com/Dhruvesh08/mecha-accelerometer.git;protocol=https;nobranch=1;branch=main"
SRCREV = "eb7820b3afc0764c968603f8a1c56f120b0e7b8d"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+eb7820b3af"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
"



# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "mecha-accelerometer"
HOMEPAGE = "https://github.com/Dhruvesh08/mecha-accelerometer.git"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include mecha-accelerometer-${PV}.inc
include mecha-accelerometer.inc
