use std::{time,thread};

fn main() {
    let (mut A,mut B)=(0f32,0f32);
    let (i,j):(f32,f32);
    let k:i32;
    let mut z:[f32;1760]=[0.;1760];
    let mut b:[char;1760]=[' ';1760];
    print!("\x1B[2J\x1B[1;1H");
    loop {
        b[0..1760].fill(32 as char);
        z[0..1760].fill(0.);
        let mut j=0f32;
        while j < 6.28 {
            let mut i=0f32;
            while i < 6.28 {
                let (c,d,e,f,g) = (i.sin(),j.cos(),A.sin(),j.sin(),A.cos());
                let h=d+2.;
                let D=1./(c*h*e+f*g+5.);
                let (l,m,n,t)=(i.cos(),B.cos(),B.sin(),c*h*g-f*e);
                let (x,y)=(40.+30.*D*(l*h*m-t*n),12.+15.*D*(l*h*n+t*m));
                let o=x as usize+80*y as usize;
                let N=8.*((f*e-c*d*g)*m-c*d*e-f*g-l*d*n);
                if 22.>y&&y>0.&&x>0.&&80.>x&&D>z[o]{ z[o]=D;b[o] = String::from(".,-~:;=!*#$@").chars().nth(if N>0.{N.round() as usize}else{0}).unwrap()}
                i+=0.02;
            }
            j+=0.07;
        }
        print!("\x1b[H");
        for k in 0..1760 {
            print!("{}", if k%80!=0{b[k as usize]}else{10 as char});
            A+=0.00004;
            B+=0.00002;
        }
        thread::sleep(time::Duration::from_micros(30000));
    }
}