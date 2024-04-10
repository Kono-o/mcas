use crate::log;

pub fn help() {
    log::msg("mcas get <version>     - to get the resource pack for that version");
    log::msg("mcas get 1.16.4");
}
