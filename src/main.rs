use utau_rs::*;

fn main(){
    simple_test();
    half_upper();
}

fn simple_test(){
    let mut uta_sections=UtaSections::default();
    if let Err(err)=uta_sections.write(){
        panic!("MyError：{}\n",err);
    }
}

fn half_upper(){
    let mut uta_sections=UtaSections::default();

    for section in uta_sections.sections.iter_mut(){
        section.note_num+=1;
    }

    if let Err(err)=uta_sections.write(){
        panic!("MyError：{}\n",err);
    }
}