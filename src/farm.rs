use std::iter::from_fn;
use rand::{random, Rng};
use crate::den::Den;

pub struct Farm{
    dens: Vec<Den>
}

impl Farm {
    pub fn check(&mut self, den_idx:u8) ->bool{
       let x = self.dens.get(den_idx as usize);
       let result = match x {
           None => { false }
           Some(d) => { d.has_fox }
       };
        self.mov();
        return result;
    }
    
    pub fn new()-> Farm{
        let mut f = Farm{ dens: vec![Den{ has_fox: false };5]};
        let pos =rand::thread_rng().gen_range(0..5);
        f.dens.get_mut(pos).unwrap().has_fox = true;
        return f;

    }

    pub fn fox_idx(&self)-> u8{
        let mut i = 0;
        for x in &self.dens {
            if x.has_fox {return i  }
            i += 1;
        }
        panic!("No fox!")
    }

    pub fn mov(&mut self){
        let pos = self.fox_idx();
        self.dens[pos as usize].has_fox = false;
        if pos ==0 {
            self.dens[pos as usize +1].has_fox = true;
        }else if pos == 4 {
            self.dens[pos as usize - 1].has_fox = true;
        }else{
            let num = rand::thread_rng().gen_range(0..=1);
            if num == 0 {
                self.dens[pos as usize + 1].has_fox = true;
            }else{
                self.dens[pos as usize - 1].has_fox = true;
            }
        }
    }
}