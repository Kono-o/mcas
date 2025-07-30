use crate::func;

pub fn help() {
    func::msg_ok("USAGE: mcas <func>");
    func::msg_info(" GET");
    func::msg_info("   - mcas get <version>       (save pack x.y.z)");
    func::msg_info("            ⤷ 'mcas get 1.16.4'");
    func::msg_info("   - mcas get <version> <dir> (save pack x.y.z in dir)");
    func::msg_info("            ⤷ 'mcas get 1.16.4 save/pack/to/this/dir'");
    func::msg_info("            ⤷ 'mcas get 1.16.4 +extend/current/dir'");
}
