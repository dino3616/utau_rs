use std::{io::{Read,Write,BufWriter},env,fs::*};
use encoding_rs::SHIFT_JIS;

pub struct UtaIO{
    pub tmpfile: String,
    file_data: String,
}

impl UtaIO{
    fn new()->Result<UtaIO,&'static str>{
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

        Ok(UtaIO{
            tmpfile,
            file_data,
        })
    }

    fn read(&mut self)->Result<(),&'static str>{
        let file_byte=match UtaIO::read_file_as_byte(&self.tmpfile){
            Ok(ok)=>ok,
            Err(err)=>match err{
                1=>return Err("ファイルを開けませんでした."),
                _=>return Err("不明なエラーが発生しました."),
            }
        };
        let (decoded,_,_)=SHIFT_JIS.decode(&file_byte);
        self.file_data=decoded.into_owned();

        Ok(())
    }

    fn write(&self)->Result<(),&'static str>{
        let mut buf=Vec::new();
        for line in self.file_data.split("\n"){
            let line=&format!("{}\n",line)[..];
            let (encoded,_,_)=SHIFT_JIS.encode(line);
            buf.push(encoded.into_owned());
        }

        if let Err(err)=UtaIO::write_file_as_byte(&buf,&self.tmpfile[..]){
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
}

impl Default for UtaIO{
    fn default()->Self{
        match UtaIO::new(){
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error：{}\n",err);
                std::process::exit(1);
            }
        }
    }
}

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
    pub uta_io: UtaIO,
    pub setting: String,
    pub prev: String,
    pub sections: Vec<UtaData>,
    pub next: String,
}

impl UtaSections{
    pub fn new()->Result<UtaSections,&'static str>{
        Ok(UtaSections{
            uta_io: UtaIO::new()?,
            setting: String::new(),
            prev: String::new(),
            sections: Vec::new(),
            next: String::new(),
        })
    }

    pub fn read(&mut self)->Result<(),String>{
        self.uta_io.read()?;

        for section in self.uta_io.file_data.split("[#"){
            match section{
                ""=>continue,
                _=>(),
            }

            let mut one_section=UtaData::new();

            let section=format!("[#{}",section);
            let mut lines=section.split("\n");

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
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
                one_section.section_name=line.chars().enumerate().filter(|&(i,_)|i>="[#".chars().count()&&i<"[#0000".chars().count()).fold("".to_string(),|s,(_,c)|format!("{}{}",s,c));
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
            };
            if line.starts_with("Length="){
                let (_,line)=line.split_whitespace().next().unwrap().split_at("Length=".chars().count());
                one_section.length=match line.parse(){
                    Ok(ok)=>ok,
                    Err(_)=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
                };
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
            };
            if line.starts_with("Lyric="){
                one_section.lyric=line.chars().skip("Lyric=".chars().count()).take(line.chars().count()).collect();
            }

            let line=match lines.next(){
                Some(some)=>some,
                None=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
            };
            if line.starts_with("NoteNum="){
                let (_,line)=line.split_whitespace().next().unwrap().split_at("NoteNum=".chars().count());
                one_section.note_num=match line.parse(){
                    Ok(ok)=>ok,
                    Err(_)=>return Err(format!("テンポラリファイルが破損しています. {}:{}",file!(),line!())),
                };
            }

            loop{
                let line=match lines.next(){
                    Some(some)=>some,
                    None=>break,
                };
                one_section.others=format!("{}{}\n",one_section.others,line);
            }

            self.sections.push(one_section);
        }

        Ok(())
    }

    pub fn write(&mut self)->Result<(),&'static str>{
        self.uta_io.file_data=self.apply()?;
        self.uta_io.write()?;

        Ok(())
    }

    fn apply(&self)->Result<String,&'static str>{
        let mut buf=format!("{}{}",self.setting,self.prev);
        for uta_data in &self.sections{
            buf=format!("{}[#{}]\nLength={}\nLyric={}\nNoteNum={}\n{}",buf,uta_data.section_name,uta_data.length,uta_data.lyric,uta_data.note_num,uta_data.others);
        }
        buf=format!("{}{}",buf,self.next);

        Ok(buf)
    }
}

impl Default for UtaSections{
    fn default()->Self{
        let mut uta_sections=match UtaSections::new(){
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error：{}\n",err);
                std::process::exit(1);
            }
        };
        if let Err(err)=uta_sections.read(){
            eprint!("Error：{}\n",err);
            std::process::exit(1);
        }

        uta_sections
    }
}