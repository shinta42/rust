

/* -*- mode: rust; indent-tabs-mode: nil -*-
 * Implementation of 'fasta' benchmark from
 * Computer Language Benchmarks Game
 * http://shootout.alioth.debian.org/
 */
use std;
import std::vec;
import std::str;
import std::uint;
import std::int;

fn LINE_LENGTH() -> uint { ret 60u; }

obj myrandom(mutable u32 last) {
    fn next(u32 mx) -> u32 {
        last = (last * 3877u32 + 29573u32) % 139968u32;
        auto ans = mx * last / 139968u32;
        ret ans;
    }
}

type aminoacids = rec(char ch, u32 prob);

fn make_cumulative(vec[aminoacids] aa) -> vec[aminoacids] {
    let u32 cp = 0u32;
    let vec[aminoacids] ans = [];
    for (aminoacids a in aa) { cp += a.prob; ans += [rec(ch=a.ch, prob=cp)]; }
    ret ans;
}

fn select_random(u32 r, vec[aminoacids] genelist) -> char {
    if (r < genelist.(0).prob) { ret genelist.(0).ch; }
    fn bisect(vec[aminoacids] v, uint lo, uint hi, u32 target) -> char {
        if (hi > lo + 1u) {
            let uint mid = lo + (hi - lo) / 2u;
            if (target < v.(mid).prob) {
                be bisect(v, lo, mid, target);
            } else { be bisect(v, mid, hi, target); }
        } else { ret v.(hi).ch; }
    }
    ret bisect(genelist, 0u, vec::len[aminoacids](genelist) - 1u, r);
}

fn make_random_fasta(str id, str desc, vec[aminoacids] genelist, int n) {
    log ">" + id + " " + desc;
    auto rng = myrandom(std::rand::mk_rng().next());
    let str op = "";
    for each (uint i in uint::range(0u, n as uint)) {
        str::push_byte(op, select_random(rng.next(100u32), genelist) as u8);
        if (str::byte_len(op) >= LINE_LENGTH()) { log op; op = ""; }
    }
    if (str::byte_len(op) > 0u) { log op; }
}

fn make_repeat_fasta(str id, str desc, str s, int n) {
    log ">" + id + " " + desc;
    let str op = "";
    let uint sl = str::byte_len(s);
    for each (uint i in uint::range(0u, n as uint)) {
        str::push_byte(op, s.(i % sl));
        if (str::byte_len(op) >= LINE_LENGTH()) { log op; op = ""; }
    }
    if (str::byte_len(op) > 0u) { log op; }
}

fn acid(char ch, u32 prob) -> aminoacids { ret rec(ch=ch, prob=prob); }

fn main(vec[str] args) {
    let vec[aminoacids] iub =
        make_cumulative([acid('a', 27u32), acid('c', 12u32), acid('g', 12u32),
                         acid('t', 27u32), acid('B', 2u32), acid('D', 2u32),
                         acid('H', 2u32), acid('K', 2u32), acid('M', 2u32),
                         acid('N', 2u32), acid('R', 2u32), acid('S', 2u32),
                         acid('V', 2u32), acid('W', 2u32), acid('Y', 2u32)]);
    let vec[aminoacids] homosapiens =
        make_cumulative([acid('a', 30u32), acid('c', 20u32), acid('g', 20u32),
                         acid('t', 30u32)]);
    let str alu =
        "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGG" +
            "GAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGA" +
            "CCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAAT" +
            "ACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCA" +
            "GCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGG" +
            "AGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCC" +
            "AGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";
    let int n = 512;
    make_repeat_fasta("ONE", "Homo sapiens alu", alu, n * 2);
    make_random_fasta("TWO", "IUB ambiguity codes", iub, n * 3);
    make_random_fasta("THREE", "Homo sapiens frequency", homosapiens, n * 5);
}