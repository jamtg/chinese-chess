use std::ops::Index;
use std::process::Command;
extern crate wincolor;
use wincolor::{Console, Color, Intense};
use std::io::{self, Write};
extern crate term_cursor as cursor;
#[derive(Copy, Clone,Debug,PartialEq)]
enum ALLChess {
    C,M,X,S,J,P,Z,N,
}
#[derive(Copy, Clone,Debug,PartialEq)]
enum Who {
    R,B,G,
}
#[derive(Copy, Clone,Debug)]
struct OneChess {
    name: ALLChess,
    w: Who,
}

fn main() {

    //println!("Hello, world!");
    let nochess = OneChess{
        name: ALLChess::N,
        w: Who::G,
    };
    let four_chess =
        [ALLChess::C,
        ALLChess::M,
        ALLChess::X,
        ALLChess::S,];
    let mut r_nine_chess:[OneChess;9] = [nochess;9];
    for i in 0..4 {
        let s = OneChess{
            name: *four_chess.index(i),
            w: Who::R,
        };
        r_nine_chess[i] = s;
    }
    r_nine_chess[4] = OneChess{
        name: ALLChess::J,
        w: Who::R,
    };
    for i in 0..4 {
        let s = OneChess{
            name: *four_chess.index(3-i),
            w: Who::R,
        };
        r_nine_chess[i+5] = s;
    }

    let mut lattice:Vec<OneChess> = vec![];
    for i in r_nine_chess.iter() {
        lattice.push(*i);
    }

    for i in 0..72 {
        lattice.push(nochess);
    }

    for i in 0..9 {
        let mut s = r_nine_chess[i];
        s.w = Who::B;
        lattice.push(s);
    }

    lattice[19] = OneChess{
        name: ALLChess::P,
        w: Who::R,
    };
    lattice[25] = OneChess{
        name: ALLChess::P,
        w: Who::R,
    };
    lattice[64] = OneChess{
        name: ALLChess::P,
        w: Who::B,
    };
    lattice[70] = OneChess{
        name: ALLChess::P,
        w: Who::B,
    };
    for i in 0..9 {
        if i%2 == 0 {
            let s1 = OneChess{
                name: ALLChess::Z,
                w: Who::R,
            };
            let s2 = OneChess{
                name: ALLChess::Z,
                w: Who::B,
            };
            lattice[i+27] = s1;
            lattice[i+54] = s2;
        }
    }

    pl(&lattice);
    println!("Enter like this:1 9 2 7");
    println!("Enter q to quit");
    loop {
        print!("input:");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                //println!("{} bytes read", n);
                //println!("{}", input);
                if input == "q\r\n" {
                    break;
                }
                let mut l:Vec<usize> = vec![];
                for i in input.bytes() {
                    //println!("{}", i as usize);
                    l.push(i as usize);
                }
                let (x1,y1,x2,y2) = (l[0]-48,l[2]-48,l[4]-48,l[6]-48);
                //println!("{}{}{}{}", x1,y1,x2,y2);
                let t = j(&lattice,(x1,y1),(x2,y2));
                //println!("{}",t);
                if t {
                    mo(&mut lattice,(x1,y1),(x2,y2));
                    clear();
                    pl(&lattice);
                }

            }
            Err(error) => println!("error: {}", error),
        }

    }


}
fn clear(){
    cursor::clear().expect("Clear failed");
    //cursor::set_pos(0, 0).expect("Setting the cursor position failed");
}
fn mo(a:&mut Vec<OneChess>,f:(usize,usize),t:(usize,usize)){
    println!("{:?}{:?}",a[f.0+f.1*9],a[t.0+t.1*9]);
    let nochess = OneChess{
        name: ALLChess::N,
        w: Who::G,
    };
    a[t.0+t.1*9] = a[f.0+f.1*9];
    a[f.0+f.1*9] = nochess;
    println!("{:?}{:?}",a[f.0+f.1*9],a[t.0+t.1*9]);
}
fn pl(a:&Vec<OneChess>){
    let mut con = Console::stdout().unwrap();
    con.fg(Intense::Yes, Color::White).unwrap();
    println!(" 0 1 2 3 4 5 6 7 8");
    print!("0");
    io::stdout().flush().unwrap();
    for i in 0..90 {
        //print!("{:?}",a[i].name);
        match a[i].w {
            Who::R => {
                con.fg(Intense::Yes, Color::Red).unwrap();
                match a[i].name {
                    ALLChess::C => print!("车"),
                    ALLChess::M => print!("马"),
                    ALLChess::X => print!("相"),
                    ALLChess::S => print!("士"),
                    ALLChess::J => print!("帅"),
                    ALLChess::P => print!("炮"),
                    ALLChess::Z => print!("兵"),
                    ALLChess::N => {},
                }
                io::stdout().flush().unwrap();
            },
            Who::B => {
                con.fg(Intense::Yes, Color::Blue).unwrap();
                match a[i].name {
                    ALLChess::C => print!("车"),
                    ALLChess::M => print!("马"),
                    ALLChess::X => print!("象"),
                    ALLChess::S => print!("士"),
                    ALLChess::J => print!("将"),
                    ALLChess::P => print!("炮"),
                    ALLChess::Z => print!("卒"),
                    ALLChess::N => {},
                }
                io::stdout().flush().unwrap();
            },
            Who::G => {
                con.fg(Intense::Yes, Color::White).unwrap();
                print!("口");
                io::stdout().flush().unwrap();
            },
        }
        let c = (i+1)/9;
        if (i+1)%9 == 0 && c != 10{
            println!("");
            if i == 44 {
                con.fg(Intense::Yes, Color::Red).unwrap();
                print!("    楚河   ");
                io::stdout().flush().unwrap();
                con.fg(Intense::Yes, Color::Blue).unwrap();
                println!("汉界  ");
            }
            con.fg(Intense::Yes, Color::White).unwrap();
            print!("{}",c);
            io::stdout().flush().unwrap();
        }


    }
    println!("");
}

fn isN(a:ALLChess) -> bool {
   if a == ALLChess::N { return true};
    false
}
fn isSome(a:Who,b:Who) -> bool {
    if a == b { return true};
    false
}
fn area (f:(usize,usize),t:(usize,usize)) -> (usize,usize,usize) {
    let mut x:usize = 0;
    let mut y:usize = 0;
    if f.0 > t.0 {
        x = f.0 - t.0 + 1;
    } else {
        x = t.0 - f.0 +1;
    }
    if f.1 > t.1 {
        y = f.1 - t.1 + 1;
    } else {
        y = t.1 - f.1 +1;
    }
    return (x*y,x,y);
}
fn j(a:&Vec<OneChess>,f:(usize,usize),t:(usize,usize)) -> bool {
    if isN(a[f.0+f.1*9].name) {
        return false;
    }
    if !isN(a[t.0+t.1*9].name) && isSome(a[f.0+f.1*9].w,a[t.0+t.1*9].w) {
        return false;
    }
    match a[f.0+f.1*9].name {
        ALLChess::C => {
            if f.0!= t.0 {
                if f.1!= t.1 {
                    return false;
                } else {
                    for i in f.0+1..t.0 {
                        if !isN(a[i+f.1*9].name) {
                            return false;
                        }
                    }
                };
            } else {
                for i in f.1+1..t.1 {
                    if !isN(a[f.0+i*9].name) {
                        return false;
                    }
                }
            };
            //println!("{}",area(f,t));
        },
        ALLChess::M => {
            let (xy,x,y) = area(f,t);
            if  xy!= 6 {
                return false;
            }
            match x {
                2 => {
                    if f.1 <= 1 {
                        if !isN(a[f.0+(f.1+1)*9].name) {
                            return false;
                        }
                    } else if f.1 >= 8 {
                        if !isN(a[f.0+(f.1-1)*9].name) {
                            return false;
                        }
                    } else {
                        if !isN(a[f.0+(f.1+1)*9].name) {
                            return false;
                        }
                        if !isN(a[f.0+(f.1-1)*9].name) {
                            return false;
                        }
                    }
                },
                3 => {
                    if f.0 <= 1 {
                        if !isN(a[f.0+1+f.1*9].name) {
                            return false;
                        }
                    } else if f.0 >= 7 {
                        if !isN(a[f.0-1+f.1*9].name) {
                            return false;
                        }
                    } else {
                        if !isN(a[f.0+1+f.1*9].name) {
                            return false;
                        }
                        if !isN(a[f.0-1+f.1*9].name) {
                            return false;
                        }
                    }
                },
                _ => return false,
            }

        },
        ALLChess::X => {
            match (f,t) {
                ((2,0),(0,2)) | ((0,2),(2,0))
                => if !isN(a[10].name) {
                return false;
                },
                ((0,2),(2,4)) | ((2,4),(0,2))
                => if !isN(a[28].name) {
                    return false;
                },
                ((2,4),(4,2)) | ((4,2),(2,4))
                => if !isN(a[30].name) {
                    return false;
                },
                ((4,2),(2,0)) | ((2,0),(4,2))
                => if !isN(a[12].name) {
                    return false;
                },
                ((4,2),(7,0)) | ((7,0),(4,2))
                => if !isN(a[14].name) {
                    return false;
                },
                ((7,0),(8,2)) | ((8,2),(7,0))
                => if !isN(a[16].name) {
                    return false;
                },
                ((8,2),(7,4)) | ((7,4),(8,2))
                => if !isN(a[34].name) {
                    return false;
                },
                ((7,4),(4,2)) | ((4,2),(7,4))
                => if !isN(a[22].name) {
                    return false;
                },

                ((2,9),(0,7)) | ((0,7),(2,9))
                => if !isN(a[73].name) {
                    return false;
                },
                ((0,7),(2,5)) | ((2,5),(0,7))
                => if !isN(a[55].name) {
                    return false;
                },
                ((2,5),(4,7)) | ((4,7),(2,5))
                => if !isN(a[57].name) {
                    return false;
                },
                ((4,7),(2,9)) | ((2,9),(4,7))
                => if !isN(a[75].name) {
                    return false;
                },
                ((4,7),(6,9)) | ((6,9),(4,7))
                => if !isN(a[78].name) {
                    return false;
                },
                ((6,9),(8,7)) | ((8,7),(6,9))
                => if !isN(a[79].name) {
                    return false;
                },
                ((8,7),(6,5)) | ((6,5),(8,7))
                => if !isN(a[61].name) {
                    return false;
                },
                ((6,5),(4,7)) | ((4,7),(6,5))
                => if !isN(a[59].name) {
                    return false;
                },

                _ => return false,

            }

        },
        ALLChess::S => {
            match (f,t) {
                ((3,0), (4,1)) | ((4,1), (3,0)) => return true,
                ((4,1), (5,0)) | ((5,0), (4,1)) => return true,
                ((4,1), (3,2)) | ((3,2), (4,1)) => return true,
                ((4,1), (5,2)) | ((5,2), (4,1)) => return true,

                ((3,9), (4,8)) | ((4,8), (3,9)) => return true,
                ((4,8), (5,9)) | ((5,9), (4,8)) => return true,
                ((4,8), (3,7)) | ((3,7), (4,8)) => return true,
                ((4,8), (5,7)) | ((5,7), (4,8)) => return true,
                _ => return false,
            }
        },
        ALLChess::J => {
            if f.0 == t.0 && a[t.0+t.1*9].name == ALLChess::J {
                for i in f.1+1..t.1 {
                    if !isN(a[f.0+i*9].name) {
                        return false;
                    }
                }
            }
            if t.0 <= 3 || t.0 >= 7 { return false };
            if t.1 <= 6 { return false };
            let (xy,x,y) = area(f,t);
            if  xy!= 2 {
                return false;
            }

        },
        ALLChess::P => {
            let mut c = 0;
            if f.0!= t.0 {
                if f.1!= t.1 {
                    return false;
                } else {
                    for i in f.0+1..t.0 {
                        if !isN(a[i+f.1*9].name) {
                            c += 1;
                        }
                    }
                    if c != 1 { return false; }
                };
            } else {
                for i in f.1+1..t.1 {
                    if !isN(a[i+f.1*9].name) {
                        c += 1;
                    }
                }
                if c != 1 { return false; }
            };
        },
        ALLChess::Z => {
            let (xy,x,y) = area(f,t);
            if  xy!= 2 {
                return false;
            }
            if a[f.0+f.1*9].w == a[0].w {
                if t.1 < f.1 { return false; }
                if f.1 <= 4 && y != 2 { return false; }
            }
            if a[f.0+f.1*9].w == a[89].w {
                if t.1 > f.1 { return false; }
                if f.1 >= 5 && y != 2 { return false; }
            }



        },
        _ => return false,
    }
    true
}