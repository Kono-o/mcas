use crate::log;

pub fn help() {
    log::msg("run [mcas get <version>] to get the resource pack for that version");
    log::msg("example: mcas get 1.16.4")
}
