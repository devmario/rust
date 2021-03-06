// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast

extern mod std;

fn start(c: &comm::Chan<comm::Chan<~str>>) {
    let (p, ch) = comm::stream();
    c.send(ch);

    let mut a;
    let mut b;
    a = p.recv();
    fail_unless!(a == ~"A");
    log(error, a);
    b = p.recv();
    fail_unless!(b == ~"B");
    log(error, b);
}

pub fn main() {
    let (p, ch) = comm::stream();
    let child = task::spawn(|| start(&ch) );

    let c = p.recv();
    c.send(~"A");
    c.send(~"B");
    task::yield();
}
