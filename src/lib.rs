use std::{io::{Read,Write,BufWriter},env,fs::*};
use encoding_rs::SHIFT_JIS;

// pub struct UtaIO{
//     pub tmpfile: String,
//     pub file_data: String,
// }

// impl UtaIO{
//     pub fn new()->Result<UtaIO,&'static str>{
//         let args: Vec<String>=env::args().collect();
//         let tmpfile;
//         if let Some(some)=args.get(1){
//             tmpfile=&some[..];
//         }
//         else{
//             return Err("UTAUから正常にファイルパスが渡されませんでした.");
//         }

//         let tmpfile=tmpfile.to_string();
//         let file_data=String::new();

//         Ok(UtaIO{
//             tmpfile,
//             file_data,
//         })
//     }

//     pub fn read(&mut self)->Result<(),&'static str>{
//         let file_byte=match UtaIO::read_file_as_byte(&self.tmpfile){
//             Ok(ok)=>ok,
//             Err(err)=>match err{
//                 1=>return Err("ファイルを開けませんでした."),
//                 _=>return Err("不明なエラーが発生しました."),
//             }
//         };
//         let (decoded,_,_)=SHIFT_JIS.decode(&file_byte);
//         self.file_data=decoded.into_owned();

//         Ok(())
//     }

//     pub fn write(&self)->Result<(),&'static str>{
//         let mut buf=Vec::new();
//         for line in self.file_data.split("\n"){
//             let line=&format!("{}\n",line)[..];
//             let (encoded,_,_)=SHIFT_JIS.encode(line);
//             buf.push(encoded.into_owned());
//         }

//         if let Err(err)=UtaIO::write_file_as_byte(&buf,&self.tmpfile[..]){
//             match err{
//                 1=>return Err("ファイルを作成できませんでした."),
//                 _=>return Err("不明なエラーが発生しました."),
//             }
//         }

//         Ok(())
//     }

//     fn read_file_as_byte(filename: &str)->Result<Vec<u8>,u8>{
//         let mut file=match File::open(filename){
//             Ok(ok)=>ok,
//             Err(_)=>{
//                 return Err(1);
//             }
//         };

//         let metadata=metadata(&filename).unwrap();
//         let mut buffer=vec![0;metadata.len() as usize];
//         file.read(&mut buffer).unwrap();
    
//         Ok(buffer)
//     }

//     fn write_file_as_byte(buf: &Vec<Vec<u8>>,filename: &str)->Result<(),u8>{
//         let mut writer=BufWriter::new(match File::create(filename){
//             Ok(ok)=>ok,
//             Err(_)=>return Err(1),
//         });

//         for datum in buf{
//             writer.write_all(&datum).unwrap();
//         }
//         writer.flush().unwrap();

//         Ok(())
//     }
// }

// impl Default for UtaIO{
//     fn default()->Self{
//         let mut default=match UtaIO::new(){
//             Ok(ok)=>ok,
//             Err(err)=>{
//                 eprint!("Error：{}\n",err);
//                 std::process::exit(1);
//             }
//         };
//         if let Err(err)=default.read(){
//             eprint!("Error：{}\n",err);
//             std::process::exit(1);
//         }

//         default
//     }
// }

pub struct UtaData{
    pub section_name: String,
    pub length: u32,
    pub lyric: String,
    pub note_num: u32,
    pub others: String,
}

impl UtaData{
    fn new()->UtaData{
        UtaData{
            section_name: String::new(),
            length: 0,
            lyric: String::new(),
            note_num: 0,
            others: String::new(),
        }
    }
}

pub struct UtaSections{
    pub tmpfile: String,
    pub file_data: String,
    pub setting: String,
    pub prev: String,
    pub sections: Vec<UtaData>,
    pub next: String,
}

impl UtaSections{
    pub fn new()->Result<UtaSections,&'static str>{
        let args: Vec<String>=env::args().collect();
        let tmpfile;
        if let Some(some)=args.get(1){
            tmpfile=&some[..];
        }
        else{
            return Err("UTAUから正常にファイルパスが渡されませんでした.");
        }

        let tmpfile=tmpfile.to_string();
        let file_data=String::new();

        Ok(UtaSections{
            tmpfile,
            file_data,
            setting: String::new(),
            prev: String::new(),
            sections: Vec::new(),
            next: String::new(),
        })
    }

    pub fn read(&mut self)->Result<(),&'static str>{
        let file_byte=match UtaSections::read_file_as_byte(&self.tmpfile){
            Ok(ok)=>ok,
            Err(err)=>match err{
                1=>return Err("ファイルを開けませんでした."),
                _=>return Err("不明なエラーが発生しました."),
            }
        };
        let (decoded,_,_)=SHIFT_JIS.decode(&file_byte);
        self.file_data=decoded.into_owned();

        print!("{}",self.file_data);

        for section in self.file_data.split("[#"){
            match section{
                ""=>continue,
                _=>(),
            }

            let mut one_section=UtaData::new();

            let section=format!("[#{}",section);
            let mut lines=section.split("\n");

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err("テンポラリファイルが破損しています."),
            };
            if line.starts_with("[#"){
                if line.contains("SETTING"){
                    self.setting=section;
                    continue;
                }
                if line.contains("PREV"){
                    self.prev=section;
                    continue;
                }
                if line.contains("NEXT"){
                    self.next=section;
                    continue;
                }
                one_section.section_name=line["[#".len()..line.len()-2].to_string();
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err("テンポラリファイルが破損しています."),
            };
            if line.starts_with("Length="){
                one_section.length=match line["Length=".len()..line.len()-1].parse(){
                    Ok(ok)=>ok,
                    Err(_)=>return Err("テンポラリファイルが破損しています."),
                };
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err("テンポラリファイルが破損しています."),
            };
            if line.starts_with("Lyric="){
                one_section.lyric=line["Lyric=".chars().count()..line.chars().count()-2].to_string();
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err("テンポラリファイルが破損しています."),
            };
            if line.starts_with("NoteNum="){
                one_section.note_num=match line["NoteNum=".len()..line.len()-1].parse(){
                    Ok(ok)=>ok,
                    Err(_)=>return Err("テンポラリファイルが破損しています."),
                }
            }

            loop{
                let line=match lines.next(){
                    Some(some)=>some,
                    None=>break,
                };
                one_section.others=format!("{}{}\n",one_section.others,line);
            }
            one_section.others=one_section.others[..one_section.others.len()-1].to_string();

            self.sections.push(one_section);
        }

        Ok(())
    }

    pub fn write(&mut self)->Result<(),&'static str>{
        self.apply()?;

        let mut buf=Vec::new();
        for line in self.file_data.split("\n"){
            let line=&format!("{}\n",line)[..];
            let (encoded,_,_)=SHIFT_JIS.encode(line);
            buf.push(encoded.into_owned());
        }

        if let Err(err)=UtaSections::write_file_as_byte(&buf,&self.tmpfile[..]){
            match err{
                1=>return Err("ファイルを作成できませんでした."),
                _=>return Err("不明なエラーが発生しました."),
            }
        }

        Ok(())
    }

    fn read_file_as_byte(filename: &str)->Result<Vec<u8>,u8>{
        let mut file=match File::open(filename){
            Ok(ok)=>ok,
            Err(_)=>{
                return Err(1);
            }
        };

        let metadata=metadata(&filename).unwrap();
        let mut buffer=vec![0;metadata.len() as usize];
        file.read(&mut buffer).unwrap();
    
        Ok(buffer)
    }

    fn write_file_as_byte(buf: &Vec<Vec<u8>>,filename: &str)->Result<(),u8>{
        let mut writer=BufWriter::new(match File::create(filename){
            Ok(ok)=>ok,
            Err(_)=>return Err(1),
        });

        for datum in buf{
            writer.write_all(&datum).unwrap();
        }
        writer.flush().unwrap();

        Ok(())
    }

    fn apply(&mut self)->Result<(),&'static str>{
        let mut buf=format!("{}{}",self.setting,self.prev);
        for uta_data in &self.sections{
            buf=format!("{}[#{}]\nLength={}\nLyric={}\nNoteNum={}\n{}",buf,uta_data.section_name,uta_data.length,uta_data.lyric,uta_data.note_num,uta_data.others);
        }
        buf=format!("{}{}",buf,self.next);

        self.file_data=buf;

        Ok(())
    }
}

impl Default for UtaSections{
    fn default()->Self{
        let mut default=match UtaSections::new(){
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error: {}",err);
                std::process::exit(1);
            }
        };
        if let Err(err)=default.read(){
                eprint!("Error: {}",err);
                std::process::exit(1);
        };

        default
    }
}