//! Lifetimes, ownership and borrowing;

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

#[test]
fn test_sat_01() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

/// CubeSate
/// #Examples
/// ```
/// let a = CubeSate{ id: 1024 };
/// ```
/// Encountering our first lifetime issue here.
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

/// check `CubeSat` status
fn check_status_01(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[test]
fn test_sat_02() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_status_01(sat_a);
    let b_status = check_status_01(sat_b);
    let c_status = check_status_01(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // compile error for move;
    // let a_status = check_status_01(sat_a);
    // let b_status = check_status_01(sat_b);
    // let c_status = check_status_01(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

// general strategies;
#[derive(Debug)]
struct CubeSat2 {
    id: u64,
    // hold message from GroundStation
    mailbox: Mailbox,
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat2, msg: Message) {
        to.mailbox.messages.push(msg);
    }

    /// create a CubeSat2 instance on demand once;
    fn connect(&self, sat_id: u64) -> CubeSat2 {
        CubeSat2 {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
}

impl CubeSat2 {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[test]
fn test_sat_ground_station() {
    let ground = GroundStation;
    let mut sat_a = CubeSat2 {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);

    ground.send(&mut sat_a, Message::from("Hello There!!!"));
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("message: {:?}", msg);
}

// break things apart;
/// simulate a database operation;
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
}

#[test]
fn test_apart_sat() {
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        base.send(sat, Message::from("Hello."));
    }
}
