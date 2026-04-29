use std:: io;
mod story;
fn get_input() -> i32 {
    let mut input = String:: new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let input: i32 = match input.trim().parse()  {
        Ok(num) => num,
        Err(_) => 0,
    };
    input
}
struct Slime {
        hunger: i32,
        clean: i32,
        happy: i32,
        hung: i32,
        turn: i32,
    }
    fn ending(slime: &mut Slime) -> bool  {
        if slime.turn == 30 {
            print!("시스템: 슬라임이 힘을 각성 할려합니다. 엔딩 루트를 선택하세요.");
            println!("\n===============================");
            println!("1. 전생했더니 슬라임이었던 건에 대하여 | 2. 멍청한줄 알았던 슬라임 알고보니 슬라임 용사? | 3. 이 힘은... 마왕? | 4. 일반 루트?? | 5. ???");
            let end = get_input();
            match end {
                1 => { story::num_1_end(slime);
                    return false;
            }
                _ =>  { println!("없는 엔딩 입니다."); 
                        return false;
        }
            }
        }
        return true;
    }
fn play_slime(slime: &Slime ) -> bool {
    if slime.happy >= 40 {
        println!("\n===============================");
        println!("슬라임: 드디어 내가 회사에 취업 했어 그동안 고마웠어!");
        println!("일반 해피엔딩 달성!");
        println!("===============================");
        return false;
    }
    if slime.hunger <= 0 {
        println!("(X _ x) 슬라임이 너무 배가 고파서 죽었습니다... 게임 오버");
        return false;
    } else if slime.clean <= 0 {
        println!("방이 너무 더러워 병에 걸려 죽었습니다... 게임 오버");
        return false;
    } else if slime.hunger > 15 {
        println!("슬라임이 밥을 너무 많이 먹어서 배가 터져버렸습니다... 게임 오버");
        return false;
    } else if slime.happy <= 0 {
        println!("슬라임이 우울하여 죽었습니다... 게임 오버");
        return false;
    } else if slime.turn >= 7 && slime.happy <= 3 {
        println!("재미 없는 슬생... (슬라임이 가출 했습니다.)");
        return false;
    } else {
        return true;
    }
}

fn status(slime: &Slime) {
    println!("\n===============================");
    println!("현재 포만감: {} | 현재 청결도: {} | 현재 행복도: {} | 생존: {}일차",slime.hunger, slime.clean, slime.happy,slime.turn);
    println!("무엇을 할까요?");
    println!("1. 밥 주기 (청결도 -2, 포만감 +4)");
    println!("2. 놀아 주기 (포만감 -2, 행복도 +3, 청결도 -2)");
    println!("3. 잠재우기 (게임 종료)");
    println!("4. 방 청소하기 포만감 -2, 행복도 -1 ,청결도 10으로 초기화");
    println!("===============================");
}
fn main() {

    let mut my_slime = Slime {
        hunger: 10,
        clean: 10,
        happy: 10,
        hung: 0,
        turn: 1,
    };
    println!("짜잔 귀여운 슬라임이 태어났습니다 열심히 키워 보세요.");
    loop {
        if ending(&mut my_slime) == false {
            break;
        }
            if play_slime(&my_slime) == false {
                break;
            }
            status(&my_slime);
            let action = get_input();
            
            match action {
                1 => {
                    my_slime.hunger = my_slime.hunger + 4;
                    my_slime.clean = my_slime.clean - 2;
                    my_slime.hung = my_slime.hung + 1;
                        
                        if my_slime.hung % 2 == 0 {
                        my_slime.happy = my_slime.happy -1;
                        println!("( 냠냠 ) 밥을 먹었지만 계속 밥만 먹어서 지루합니다.");
                    } else {
                    println!("( 냠냠 ) 밥을 먹었습니다.");
                }
                my_slime.turn += 1; }
                2 => {
                    my_slime.hunger = my_slime.hunger - 2;
                    my_slime.happy = my_slime.happy + 3;
                    my_slime.clean = my_slime.clean -2;        
                        my_slime.turn += 1;
                    println!("( 히히히 ) 재밌게 놀았습니다.");
                } 
                3 => {
                    println!("잘 자 (게임을 종료합니다.)");
                    break;
                }
                4 => {
                    my_slime.hunger = my_slime.hunger -2;
                    my_slime.clean = 10;
                    my_slime.happy = my_slime.happy -1;
                    my_slime.turn += 1;
                    println!("( 재밌는 방 청소 )방을 청소합니다.");
                }
                _ => println!("없는 선택지 입니다."),
            }
        }
    }

