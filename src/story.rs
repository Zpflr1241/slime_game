use crate::Slime;
use crate::get_input_2;
use std::thread;
use std::time::Duration;


pub fn story(slime: &Slime)  {
    thread::sleep(Duration::from_secs(2));
    println!("슬라임: 아오... 머리야");
    thread::sleep(Duration::from_secs(5));
    println!("\n===============================");
    println!("슬라임: 제작자..두고봐 아주그냥 콱🔥");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 그리고 갑자기 무슨 장르변경이야?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 하...어이가 없네 그리고 여기는 어디야?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("(주변을 둘러보니 온통 숲 이다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 나도 알거든? 그리고 해설은 어디서 나오는거야?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: ...일단 앞으로 가볼까");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("(앞으로 이동하니 저 멀리 큰 성이 보였다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 저 성은 뭐지? 가볼까? (yes 또는 no | 입력)");
    println!("\n===============================");
    loop {
    let input = get_input_2();
    match input.as_str() {
        "yes" => {
            crate:: story_yes_1::story_battle(slime);
            break;
        }
        "no" => {
            crate:: story_no_1::story_no_1(slime);
            break;
        }
    _ => {
        println!("잘못 입력하셨습니다.");
    } 
    }
    }
}
