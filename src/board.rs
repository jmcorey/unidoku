use rand::prelude::SliceRandom;

#[derive(Debug)]
pub struct Board {
    pub nbrows:u32,
    pub nbcols:u32,
    pub nsyms:u32,
    data:Vec<u8>,
}


impl Board {

    pub fn new(nbrows:u32, nbcols:u32) -> Board {
        let nsyms = nbrows * nbcols;
        let mut b = Board {
            nbrows,
            nbcols,
            nsyms,
            data: vec![0; (nsyms * nsyms) as usize]
        };
        b.trivial_init();
        //b.scramble_syms();
        b
    }

    fn trivial_init(&mut self) -> () {
        let n = self.nsyms;
        let bc = self.nbcols;
        let br = self.nbrows;
        for r in 0..n {
            for c in 0..n {
                self.data[Self::idx(n, r, c)] =
                    ((r * bc + c + r / br) % n) as u8;
            }
        }
    }

    #[allow(dead_code)]
    fn scramble_rc(&mut self) -> () {
        // TODO scramble rows/columns
        panic!("unimplemented");
    }

    #[allow(dead_code)]
    fn scramble_syms(&mut self) -> () {
        let mut perm:Vec<u8> = (0u8..(self.nsyms as u8)).collect();
        perm.shuffle(&mut rand::thread_rng());
        for sym in self.data.iter_mut() {
            *sym = perm[*sym as usize];
            //let i = *sym as usize;
            //let j:u8 = perm[i];
            //*sym = j;
        }
    }

    fn idx(nsyms:u32, r:u32, c:u32) -> usize {
        (r * nsyms + c) as usize
    }

    pub fn get_cell(&self, r:u32, c:u32) -> u8 {
        self.data[Self::idx(self.nsyms, r, c)]
    }

    pub fn print(&self) -> () {
        for r in 0..self.nsyms {
            if r % self.nbrows == 0 {
                println!("");
            }
            for c in 0..self.nsyms {
                if c % self.nbcols == 0 {
                    print!(" ");
                }
                print!(" {:X}", self.data[Self::idx(self.nsyms, r, c)]);
            }
            println!("");
        }

    }
}
