use utau_rs::*;

fn main(){
    half_upper();
}

fn half_upper(){
    //オブジェクトの生成(TMPファイルの読み込み・各パラメータの抽出)
    let mut uta_sections=UtaSections::default();

    //セクションごとの処理
    for section in uta_sections.sections.iter_mut(){
        section.note_num+=1;
    }

    //TMPファイルへの書き込み
    if let Err(err)=uta_sections.write(){
        panic!("Error：{}\n",err);
    }
}