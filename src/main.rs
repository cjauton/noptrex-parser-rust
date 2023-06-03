#![allow(unused)]

use configparser::ini::Ini;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]

struct Pulse {
    h: Vec<u32>,
    w0: Vec<u16>,
    w1: Vec<u16>,
    w2: Vec<u16>,
    w3: Vec<u16>,
    w4: Vec<u16>,
    w5: Vec<u16>,
    w6: Vec<u16>,
    w7: Vec<u16>,
    w8: Vec<u16>
}

#[derive(Debug)]
struct Config {
    raw: i32,
    nwin: i32,
    npulses: i32,
    mask: i32,
    reclength: i32,
    posttrig: i32,
    decimation: i32,
    dcoffset: Vec<i32>,
    wchn: Vec<i32>,
    wbeg: Vec<i32>,
    wend: Vec<i32>,
    wsum: Vec<i32>,
    wdec: Vec<i32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let run = &args[1].clone();

    let filename_data = format!("./data/run{}.bin", run);
    let filename_conf = format!("./meta/run{}.conf", run);

    let config = Config::new(&filename_conf);
    read_data(&filename_data, &config);
    // println!("{:?}", config.get_wsize());
    // println!("{:?}", config);
}




fn read_data(filename: &String, config: &Config) -> Vec<Pulse> {
    println!("reading {}", filename);

    let mut f = File::open(filename).unwrap();

    let mut h_buffer: [u8; 4] = [0; 4];
    let mut w_buffer: [u8; 2] = [0; 2];

    let mut v: Vec<Pulse> = Vec::new();

    let mut n = 0;

    while n < config.npulses {

        let mut h: Vec<u32> = Vec::new();
        let mut w0: Vec<u16> = Vec::new();
        let mut w1: Vec<u16> = Vec::new();
        let mut w2: Vec<u16> = Vec::new();
        let mut w3: Vec<u16> = Vec::new();
        let mut w4: Vec<u16> = Vec::new();
        let mut w5: Vec<u16> = Vec::new();
        let mut w6: Vec<u16> = Vec::new();
        let mut w7: Vec<u16> = Vec::new();
        let mut w8: Vec<u16> = Vec::new();

        let mut container = [&mut w0, &mut w1, &mut w2, &mut w3, &mut w4, &mut w5, &mut w6, &mut w7, &mut w8];

        let mut i = 0;

        while i < 4 * 5 {
            f.read(&mut h_buffer).unwrap();
            let num = u32::from_ne_bytes(h_buffer);
            h.push(num);
            i += 4;
        }

        for (w, wsize) in config.get_wsize().iter().enumerate() {
            let mut i: i32 = 0;

            while i < 2 * wsize {
                f.read(&mut w_buffer).unwrap();
                let num = u16::from_ne_bytes(w_buffer);
                container[w].push(num);
                i += 2;
            }
        }

        println!("{:?}", container[7]);

        let p = Pulse {
            h: h,
            w0: w0,
            w1: w1,
            w2: w2,
            w3: w3,
            w4: w4,
            w5: w5,
            w6: w6,
            w7: w7,
            w8: w8
        };

        v.push(p);
        n+=1;
    }

    v
}

fn read_data2(filename: &String, config: &Config) -> Vec<Pulse> {
    println!("reading {}", filename);

    let mut f = File::open(filename).unwrap();

    let mut h_buffer: [u8; 4] = [0; 4];
    let mut w_buffer: [u8; 2] = [0; 2];

    let mut v: Vec<Pulse> = Vec::new();

    let mut n = 0;

    while n < config.npulses {

        let mut h: Vec<u32> = Vec::new();
        let mut w0: Vec<u16> = Vec::new();
        let mut w1: Vec<u16> = Vec::new();
        let mut w2: Vec<u16> = Vec::new();
        let mut w3: Vec<u16> = Vec::new();
        let mut w4: Vec<u16> = Vec::new();
        let mut w5: Vec<u16> = Vec::new();
        let mut w6: Vec<u16> = Vec::new();
        let mut w7: Vec<u16> = Vec::new();
        let mut w8: Vec<u16> = Vec::new();

        let mut container = [&mut w0, &mut w1, &mut w2, &mut w3, &mut w4, &mut w5, &mut w6, &mut w7, &mut w8];

        let mut i = 0;

        while i < 4 * 5 {
            f.read(&mut h_buffer).unwrap();
            let num = u32::from_ne_bytes(h_buffer);
            h.push(num);
            i += 4;
        }

        for (w, wsize) in config.get_wsize().iter().enumerate() {
            let mut i: i32 = 0;

            while i < 2 * wsize {
                f.read(&mut w_buffer).unwrap();
                let num = u16::from_ne_bytes(w_buffer);
                container[w].push(num);
                i += 2;
            }
        }

        // println!("{:?}", container[7]);

        let p = Pulse {
            h: h,
            w0: w0,
            w1: w1,
            w2: w2,
            w3: w3,
            w4: w4,
            w5: w5,
            w6: w6,
            w7: w7,
            w8: w8
        };

        v.push(p);
        n+=1;
    }

    v
}

impl Config {
    fn new(filename: &String) -> Config {
        println!("reading {}", filename);
        let mut config = Ini::new();
        let map = config.load(filename).unwrap();
        // println!("{:?}", map);

        let raw: i32 = config
            .get("default", "raw")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let decimation: i32 = config
            .get("default", "decimation")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let nwin: i32 = config
            .get("default", "nwin")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let npulses: i32 = config
            .get("default", "npulses")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mask: i32 = config
            .get("default", "mask")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let reclength: i32 = config
            .get("default", "reclength")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let posttrig: i32 = config
            .get("default", "posttrig")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let dcoffset: Vec<i32> = config
            .get("default", "dcoffset")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wchn: Vec<i32> = config
            .get("default", "wchn")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wbeg: Vec<i32> = config
            .get("default", "wbeg")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wend: Vec<i32> = config
            .get("default", "wend")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wbeg: Vec<i32> = config
            .get("default", "wbeg")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wsum: Vec<i32> = config
            .get("default", "wsum")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let wdec: Vec<i32> = config
            .get("default", "wdec")
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
     
       

        Config {
            raw,
            nwin,
            npulses,
            mask,
            reclength,
            posttrig,
            dcoffset,
            wchn,
            wbeg,
            wend,
            wsum,
            wdec,
            decimation,
    
        }
    }

    fn get_wsize(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        while i < self.nwin as usize {
            let size: i32 = (self.wend[i] - self.wbeg[i]) / self.wsum[i];
            v.push(size);
            i += 1;
        }

        v
    }
}
