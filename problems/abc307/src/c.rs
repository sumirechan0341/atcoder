use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        ha: usize,
        wa: usize,
        aha: [Chars; ha],
        hb: usize,
        wb: usize,
        bhb: [Chars; hb],
        hx: usize,
        wx: usize,
        xhx: [Chars; hx],
    };
    for i in 0..hx {
        for j in 0..wx {
            let mut used = HashSet::<(usize, usize)>::new();
            let mut ok = true;
            // シートAを重ね合わせる
            if xhx[i][j] == '#' {
                for k in 0..ha {
                    for l in 0..wa {
                        if i+k >= hx {
                            if aha[k][l] == '#' {
                                ok = false;
                                continue;
                            } else {
                                continue;
                            }
                        }
                        if l+j >= wx {
                            if aha[k][l] == '#' {
                                ok = false;
                                continue;
                            } else {
                                continue;
                            }
                        }
                        if aha[k][l] == '#' {
                            if xhx[i+k][j+l] != '#' {
                                ok = false;
                                continue;
                            } else {
                                used.insert((i+k, j+l));
                            }
                        }
                    }
                }
                if !ok {
                    break;
                }
                for ii in 0..hx {
                    for jj in 0..wx {
                        if xhx[ii][jj] == '#' {
                            for k in 0..hb {
                                for l in 0..wb {
                                    if ii+k >= hx {
                                        if bhb[k][l] == '#' {
                                            ok = false;
                                            continue;
                                        } else {
                                            continue;
                                        }
                                    }
                                    if l+jj >= wx {
                                        if bhb[k][l] == '#' {
                                            ok = false;
                                            continue;
                                        } else {
                                            continue;
                                        }
                                    }
                                    if bhb[k][l] == '#' {
                                        if xhx[ii+k][jj+l] != '#' {
                                            ok = false;
                                            continue;
                                        } else {
                                            used.insert((ii+k, jj+l));
                                        }
                                    }
                                }
                            }
                        } else {
                            continue;
                        }
                        let mut check = true;
                        if !ok {
                            continue;
                        }
                        for iii in 0..hx {
                            for jjj in 0..wx {
                                if xhx[iii][jjj] == '#' {
                                    if !used.contains(&(iii, jjj)) {
                                        check =false;
                                        break;
                                    }
                                }
                            }
                        }
                        if check {
                            println!("{}", "Yes");
                            return;
                        } else {
                            break;
                        }
                    }
                }
                
                if !ok {
                    continue;
                }
                

            }
        }
    }
    println!("{}", "No");

}