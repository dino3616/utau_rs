use std::process;
use utau_rs::*;

fn main(){
    let mut uta_sections=UtaSections::default();

    let selected_scale=even_scale::Scale::C;
    if let Err(err)=even_scale::even_scale(&mut uta_sections,selected_scale){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }

    if let Err(err)=uta_sections.write(){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }
}

mod even_scale{
    use utau_rs::*;

    #[allow(non_camel_case_types)]
    pub enum Scale{
        C,C_sharp,Cm,Cm_sharp,
        D,D_sharp,Dm,Dm_sharp,
        E,E_sharp,Em,Em_sharp,
        F,F_sharp,Fm,Fm_sharp,
        G,G_sharp,Gm,Gm_sharp,
        A,A_sharp,Am,Am_sharp,
        B,B_sharp,Bm,Bm_sharp,
    }
    
    pub struct KeyTone([u32;7]);
    
    const C: u32=24;
    const D: u32=26;
    const E: u32=28;
    const F: u32=29;
    const G: u32=31;
    const A: u32=33;
    const B: u32=35;
    
    impl KeyTone{
        pub fn new(scale: &Scale)->Result<KeyTone,&'static str>{
            Ok(match scale{
                Scale::C      |Scale::Am      =>KeyTone([C,D,E,F,G,A,B]),
                Scale::C_sharp|Scale::Am_sharp=>KeyTone([C+1,D+1,E+1,F+1,G+1,A+1,B+1]),
                Scale::D      |Scale::Bm      =>KeyTone([C+1,D,E,F+1,G,A,B]),
                Scale::D_sharp|Scale::Cm      =>KeyTone([C,D,E-1,F,G,A-1,B-1]),
                Scale::E      |Scale::Cm_sharp=>KeyTone([C+1,D+1,E,F+1,G+1,A,B]),
                Scale::F      |Scale::Dm      =>KeyTone([C,D,E,F,G,A,B-1]),
                Scale::F_sharp|Scale::Dm_sharp=>KeyTone([C+1,D+1,E+1,F+1,G+1,A+1,B]),
                Scale::G      |Scale::Em      =>KeyTone([C,D,E,F+1,G,A,B]),
                Scale::G_sharp|Scale::Fm      =>KeyTone([C,D-1,E-1,F,G,A-1,B-1]),
                Scale::A      |Scale::Fm_sharp=>KeyTone([C+1,D,E,F+1,G+1,A,B]),
                Scale::A_sharp|Scale::Gm      =>KeyTone([C,D,E-1,F,G,A,B-1]),
                Scale::B      |Scale::Gm_sharp=>KeyTone([C+1,D+1,E,F+1,G+1,A+1,B]),
                _=>return Err("不明なエラーが発生しました."),
            })
        }
    
        fn unwrap(&self)->[u32;7]{
            match self{
                &KeyTone(some)=>some,
            }
        }
    }
    
    pub fn even_scale(uta_sections: &mut UtaSections,scale: Scale)->Result<(),&'static str>{
        let tones=KeyTone::new(&scale).unwrap();
    
        for section in uta_sections.sections.iter_mut(){
            if tones.unwrap().iter().all(|&x|(x%section.note_num)!=0){
                let mut near=section.note_num as i32-tones.unwrap()[0] as i32;
                for tone in tones.unwrap(){
                    let tone=match section.note_num{
                        24..=35=>tone+12*0,
                        36..=47=>tone+12*1,
                        48..=59=>tone+12*2,
                        60..=71=>tone+12*3,
                        72..=83=>tone+12*4,
                        84..=95=>tone+12*5,
                        96..=107=>tone+12*6,
                        _=>return Err("不明なエラーが発生しました."),
                    };
                    if (section.note_num as i32-tone as i32).abs()<near{
                        near=(section.note_num as i32-tone as i32).abs();
                    }
                }
                section.note_num=(section.note_num as i32+near) as u32;
            }
        };
    
        Ok(())
    }
}