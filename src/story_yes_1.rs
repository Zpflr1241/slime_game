use crate::story::story;
use crate::{Slime, get_input};
use std::thread;
use std::time::Duration;

struct fight {
attack: i32,
defencd: i32,
heal: i32,
}

struct slime_status {
    hp: i32,
}

pub fn story_true_1(slime: &Slime)  {
    println!("\n===============================");
    println!("(무사히 성에 도착하였습니다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 나도 알아😡");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 이 성은 뭔데 왜이렇게 커?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("(성은 총 4층?으로 구성 되어있다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임 -> 속마음: 이쯤되면 제작자가 직접 해설 하는거 아니야?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 들어가자 일단");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(2));
    println!("(끼익)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("(성에 들어서니 커다란 오크가 있었다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 나도 알아!!! 설마 때리는거 아니지?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("오크: ???감히 마왕님에 성에 침입하다니... 용서 할 수 없다!");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 잠..잠깐 타임!");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("(그 순간 슬라임의 눈에 검과 방패가 보였다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("(슬라임은 검과 방패를 획득하였다!)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 그게 중요한게 아니잖아! 저거 쓰러뜨릴 순 있어?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: 에라 모르겠다 도망ㄱ..? (문이 굳게 닫혔다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("제작자: 👍");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("슬라임: ^^ (난 행복해요)");
    thread::sleep(Duration::from_secs(2));
    println!("슬라임: 안 행복해!!!! 야!! 제작자!!!");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 하... 어쩔수 없나...쓰러뜨리는 수 밖에");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 덤벼!");
    thread::sleep(Duration::from_secs(2));
    println!("\n===============================");
    println!("전투시작");
    thread::sleep(Duration::from_secs(2));
    println!("\n===============================");
}
pub fn story_battle(slime: &Slime) {
    story_true_1(slime);
    let war = fight {
    attack: 490, // 30
    defencd: 30,
    heal: 30,
    };
    let mut oke = 500;
    let mut slime_hp = status_2(slime);
    loop {
        println!("슬라임은 무엇을 할까요?");
        println!("===============================");
        println!("1. 기본 공격 | 2. 방어 | 3. 자가치유"); 
        let mut choice = get_input();
        let mut defending = false;
        match choice  {
            1 => {
                println!("\n===============================");
                println!("(슬라임이 공격을 날립니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                oke = oke - war.attack;
                println!("(오크가 데미지를 입었습니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("\n===============================");
                println!("오크 체력: {}", oke);
                println!("슬라임 체력: {}", slime_hp.hp);
                println!("\n===============================");
            }
            2 => {
                println!("슬라임이 방어 태세를 취합니다.");
                defending = true;
            }
            3 => {
                println!("\n===============================");
                if slime_hp.hp >= 100 {
                    println!("(체력을 회복할 수 없습니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                println!("다른 선택을 해주세요.");
                println!("슬라임 체력: {}", slime_hp.hp);
                thread::sleep(Duration::from_secs(2));
                println!("\n===============================");
                continue;
                }
                else {
                    println!("(슬라임이 자가치유를 합니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(3));
                slime_hp.hp = slime_hp.hp + 35;
                if slime_hp.hp > 200 {
                    slime_hp.hp = 200;
                }
                println!("슬라임: 휴..죽을뻔 했네..");
                println!("슬라임 체력: {}", slime_hp.hp);
                println!("\n===============================");
            }
            }
            
            
            _ => {
                println!("다시 입력해주세요.");
                continue;
            }
        }
        if oke <= 0 {
            println!("===============================");
            println!("===============================");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("오크: 나를 쓰러뜨리다니..");
            println!("===============================");
            thread::sleep(Duration::from_secs(2));
            println!("오크: 이런...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("(오크가 쓰러졌습니다.)");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임의 스탯이 상승합니다.");
            slime_hp.hp = slime_hp.hp + 100;
            println!("체력: {}",slime_hp.hp);
            thread::sleep(Duration::from_secs(2));
            break;
        }
        println!("===============================");
        println!("오크: 받아라!!");
        thread::sleep(Duration::from_secs(3));
        let oke_damage = if defending { 10 } else {30};
        slime_hp.hp = slime_hp.hp - oke_damage;
        if defending {
            println!("(슬라임은 방패로 피해를 최소화 하였습니다.) -{} 데미지", oke_damage);
        } else {
            println!("(오크의 강력한 공격을 맞았습니다.) -{} 데미지)", oke_damage);
        }
        println!("현재 슬라임 체력: {}", slime_hp.hp);
        println!("\n===============================");
        if slime_hp.hp <= 0 {
            println!("슬라임이 쓰러졌습니다...게임 오버");
            break;
        }
        }
        }
    

pub fn status_2 (slime: &Slime) -> slime_status  {
    let slime_infor = slime_status {
        hp: 200,
    };
    println!("슬라임 상태창");
    println!("===============================");
    println!("체력: {}",slime_infor.hp);
    slime_infor
    }
