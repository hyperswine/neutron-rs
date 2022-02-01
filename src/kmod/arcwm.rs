// WINDOW MANAGER INTERFACE

// WINDOW SERVER LIBRARY THAT LINKS WITH NEUTRON CORE

// Sparc = Service
use crate::services::Sparc;
use crate::services::SparcReq;

struct ArcWin {

}

impl Sparc for ArcWin {
    // observer pattern
    fn handle_req(sparc_req: SparcReq) {
        
    }

    fn start() {

    }

    fn close() {

    }
}
