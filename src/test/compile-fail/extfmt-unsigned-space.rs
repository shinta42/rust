// xfail-stage0
// error-pattern:only valid in signed #fmt conversion

fn main() {
  // Can't use a space on unsigned conversions
  #fmt("% u", 10u);
}