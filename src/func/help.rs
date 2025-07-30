use crate::func;

pub fn help() {
    func::msg_ok("USAGE: MCAS <FUNC>");
    func::msg_info(" INFO");
    func::msg_info("   - mcas info                (info about mcas)");
    func::msg_info(" GET");
    func::msg_info("   - mcas get <version>       (save pack x.y.z)");
    func::msg_info("            ⤷ 'mcas get 1.16.4'");
    func::msg_info("   - mcas get <version> <dir> (save pack x.y.z in dir)");
    func::msg_info("            ⤷ 'mcas get 1.16.4 save/pack/to/this/dir'");
    func::msg_info("            ⤷ 'mcas get 1.16.4 +extend/current/dir'");
}

pub fn info() {
    func::msg_ok("INFO: MCAS");
    func::msg_info(&format!("  name   : {}", env!("CARGO_PKG_NAME")));
    func::msg_info(&format!("  desc   : {}", env!("CARGO_PKG_DESCRIPTION")));
    func::msg_info(&format!("  version: {}", env!("CARGO_PKG_VERSION")));
    func::msg_info(&format!("  author : {}", env!("CARGO_PKG_AUTHORS")));
    func::msg_info(&format!("  license: {}", env!("CARGO_PKG_LICENSE")));
    func::msg_info(&format!("  repo   : {}", env!("CARGO_PKG_REPOSITORY")));
}
